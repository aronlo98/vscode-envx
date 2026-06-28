use std::path::PathBuf;

use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use envx::error::EnvxError;
use envx::lexer::lex_file;
use envx::{loader, parser};

pub struct Backend {
    client: Client,
    document_map: DashMap<String, String>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                definition_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "envx language server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.text_document.text;
        self.document_map.insert(uri.clone(), text.clone());
        self.validate_document(&uri, &text).await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        if let Some(change) = params.content_changes.pop() {
            let text = change.text;
            self.document_map.insert(uri.clone(), text.clone());
            self.validate_document(&uri, &text).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        self.document_map.remove(&uri);
        self.client.publish_diagnostics(params.text_document.uri, vec![], None).await;
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let path = match uri.to_file_path() {
            Ok(p) => p,
            Err(_) => return Ok(None),
        };

        // 1. Get current unsaved text
        let text = match self.document_map.get(uri.as_str()) {
            Some(t) => t.clone(),
            None => return Ok(None),
        };

        // 2. Convert position to byte offset
        let offset = position_to_byte_offset(&text, pos);

        // 3. Parse current file
        let filename = uri.path().split('/').last().unwrap_or("unknown.envx");
        let file = match parser::parse(&text, filename, path.clone()) {
            Ok(f) => f,
            Err(_) => return Ok(None),
        };

        // 4. Find var reference under cursor
        let var_name = match file.find_var_ref_at(offset) {
            Some(name) => name,
            None => return Ok(None),
        };

        // 5. Load environment (reads from disk, which is fine for cross-file)
        let env = match loader::load(&path) {
            Ok(e) => e,
            Err(_) => return Ok(None),
        };

        // 6. Look up definition
        if let Some((template, source_path)) = env.entries.get(&var_name) {
            let target_uri = match Url::from_file_path(source_path) {
                Ok(u) => u,
                Err(_) => return Ok(None),
            };

            if let Some(source_text) = env.sources.get(source_path) {
                let start_pos = byte_offset_to_position(source_text, template.span.start);
                return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                    uri: target_uri,
                    range: Range {
                        start: start_pos,
                        end: start_pos,
                    },
                })));
            }
        }

        Ok(None)
    }
}

impl Backend {
    async fn validate_document(&self, uri: &str, text: &str) {
        let mut diagnostics = vec![];
        let filename = uri.split('/').last().unwrap_or("unknown.envx");
        
        // 1. Lex the file
        match lex_file(text, filename) {
            Ok(_) => {
                // 2. Parse the file
                if let Err(e) = parser::parse(text, filename, PathBuf::from(filename)) {
                    if let Some(diag) = envx_error_to_diagnostic(&e, text) {
                        diagnostics.push(diag);
                    }
                }
            }
            Err(e) => {
                if let Some(diag) = envx_error_to_diagnostic(&e, text) {
                    diagnostics.push(diag);
                }
            }
        }
        
        let url = match Url::parse(uri) {
            Ok(u) => u,
            Err(_) => return,
        };
        self.client.publish_diagnostics(url, diagnostics, None).await;
    }
}

fn envx_error_to_diagnostic(err: &EnvxError, source: &str) -> Option<Diagnostic> {
    let span = match err {
        EnvxError::UnexpectedChar { span, .. } => span,
        EnvxError::UnterminatedString { span, .. } => span,
        EnvxError::UnclosedExpression { span, .. } => span,
        EnvxError::UnexpectedToken { span, .. } => span,
        EnvxError::EmptyExpression { span, .. } => span,
        EnvxError::UndefinedVariable { span, .. } => span,
        _ => return None,
    };
    
    let start_pos = byte_offset_to_position(source, span.offset());
    let end_pos = byte_offset_to_position(source, span.offset() + span.len());
    
    let message = format!("{}", err);
    
    Some(Diagnostic {
        range: Range { start: start_pos, end: end_pos },
        severity: Some(DiagnosticSeverity::ERROR),
        code: None,
        code_description: None,
        source: Some("envx".to_string()),
        message,
        related_information: None,
        tags: None,
        data: None,
    })
}

fn byte_offset_to_position(source: &str, offset: usize) -> Position {
    let mut line = 0;
    let mut character = 0;
    
    for (i, c) in source.char_indices() {
        if i >= offset {
            break;
        }
        if c == '\n' {
            line += 1;
            character = 0;
        } else {
            character += c.len_utf16() as u32;
        }
    }
    
    Position { line, character }
}

fn position_to_byte_offset(source: &str, pos: Position) -> usize {
    let mut current_line = 0;
    let mut current_char = 0;
    
    for (i, c) in source.char_indices() {
        if current_line == pos.line && current_char == pos.character {
            return i;
        }
        if current_line > pos.line {
            return i;
        }
        if c == '\n' {
            current_line += 1;
            current_char = 0;
        } else {
            current_char += c.len_utf16() as u32;
        }
    }
    source.len()
}

pub async fn run_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        document_map: DashMap::new(),
    });
    
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[tokio::main]
async fn main() {
    run_server().await;
}
