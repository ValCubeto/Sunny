import readline from 'readline';
import Bun from 'bun';

class SimpleShell {
    private history: string[];
    private currentLine: string;
    private cursorPosition: number;
    private autocomplete: string[];

    constructor() {
        this.history = [];
        this.currentLine = '';
        this.cursorPosition = 0;
        this.autocomplete = ['hello', 'help'];
    }

    public run(): void {
        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout,
            prompt: '> '
        });

        rl.prompt();

        rl.on('line', (line: string) => {
            if (line === '') {
                this.printPrompt();
                return;
            }

            this.currentLine = line;
            this.executeCommand();
        });

        rl.input.on('keypress', (char: string, key: readline.Key) => {
            if (!key || !key.ctrl) {
                if (key && key.name === 'return') {
                    this.executeCommand();
                } else if (key && key.name === 'tab') {
                    this.autocompleteCommand();
                } else {
                    this.insertCharacter(key ? key.name : char);
                }
            } else if (key.ctrl && key.name === 'c') {
                console.log('Goodbye!');
                process.exit();
            }
        });
    }

    private executeCommand(): void {
        if (this.currentLine.trim() !== '') {
            this.history.push(this.currentLine);
        }
        this.currentLine = '';
        this.cursorPosition = 0;
        this.printPrompt();
    }

    private printPrompt(): void {
        process.stdout.clearLine(0);
        process.stdout.cursorTo(0);
        process.stdout.write('> ' + this.currentLine);
    }

    private autocompleteCommand(): void {
        const prefix = this.currentLine.slice(0, this.cursorPosition);
        for (const word of this.autocomplete) {
            if (word.startsWith(prefix)) {
                const remaining = word.slice(prefix.length);
                this.currentLine += remaining;
                this.cursorPosition += remaining.length;
                this.printPrompt();
                break;
            }
        }
    }

    private insertCharacter(char: string): void {
        this.currentLine = this.currentLine.slice(0, this.cursorPosition) + char + this.currentLine.slice(this.cursorPosition);
        this.cursorPosition++;
        this.printPrompt();
    }
}

const shell = new SimpleShell();
shell.run();
