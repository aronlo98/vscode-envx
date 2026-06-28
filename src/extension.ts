import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    Executable
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
    const config = vscode.workspace.getConfiguration('envx');
    let executablePath = config.get<string>('executablePath');

    const isWindows = process.platform === 'win32';
    const bundledExecutableName = isWindows ? 'envx-lsp.exe' : 'envx-lsp';
    const bundledPath = context.asAbsolutePath(path.join('bin', bundledExecutableName));

    if (!executablePath || executablePath === 'envx' || executablePath === 'envx-lsp') {
        if (fs.existsSync(bundledPath)) {
            executablePath = bundledPath;
        } else {
            executablePath = 'envx-lsp';
        }
    }

    const run: Executable = {
        command: executablePath,
        options: {
            env: process.env
        }
    };

    const serverOptions: ServerOptions = {
        run,
        debug: run
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'envx' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.envx')
        }
    };

    client = new LanguageClient(
        'envxLanguageServer',
        'Envx Language Server',
        serverOptions,
        clientOptions
    );

    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
