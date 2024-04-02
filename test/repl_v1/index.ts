import readline, { Interface, Key } from 'readline';

class SimpleShell {
  private rl: Interface;
  private history: string[];
  private currentLine: string;
  private cursorPosition: number;
  private autocomplete: string[];

  constructor() {
    this.rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
      // prompt: '> '
    });

    this.history = [];
    this.currentLine = '';
    this.cursorPosition = 0;
    this.autocomplete = ['hello', 'help', 'hey', 'how'];
  }

  public run(): void {
    this.rl.on('keypress', (char: string, key: Key) => {
      if (key && key.ctrl && key.name === 'c') {
        console.log('\nGoodbye!');
        process.exit();
      }

      if (char === '\n') {
        this.executeCommand();
        return;
      }

      console.log({ name: key.name });
      if (key && key.name === 'tab') {
        this.autocompleteCommand();
      } else if (key && key.name === 'left') {
        this.moveCursorLeft();
      } else if (key && key.name === 'right') {
        this.moveCursorRight();
      } else if (key && key.name === 'home') {
        this.moveCursorToStart();
      } else if (key && key.name === 'end') {
        this.moveCursorToEnd();
      } else {
        this.insertCharacter(key?.name ?? char);
      }
    });

    // this.rl.prompt();
  }

  private executeCommand(): void {
    if (this.currentLine.trim() !== '') {
      this.history.push(this.currentLine);
    }
    this.currentLine = '';
    this.cursorPosition = 0;
    this.printPrompt();
  }

  private moveCursorLeft(): void {
    if (this.cursorPosition > 0) {
      this.cursorPosition--;
      this.printPrompt();
    }
  }

  private moveCursorRight(): void {
    if (this.cursorPosition < this.currentLine.length) {
      this.cursorPosition++;
      this.printPrompt();
    }
  }

  private moveCursorToStart(): void {
    this.cursorPosition = 0;
    this.printPrompt();
  }

  private moveCursorToEnd(): void {
    this.cursorPosition = this.currentLine.length;
    this.printPrompt();
  }

  private printPrompt(): void {
    process.stdout.clearLine(0);
    process.stdout.cursorTo(0);
    process.stdout.write('> ' + this.currentLine);
    // this.rl.prompt();
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
