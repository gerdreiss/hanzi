# 汉字

Note: this project is a work in progress!

The goal of this project is to create a simple desktop app that queries a LLM model for translation and save it to a database, to be used later for learning.

## Usage

### Before using
1. Install [Ollama](https://ollama.com/)
2. Pull a model of your choice. In my experience, mistral works best for translations.
3. Start ollama as a background service or run `ollama run <model>` or `ollama serve`

To see logs, start Hanzi providing environment variable RUST_LOG=\<log level\>

### How-to
1. Enter any chinese phrase into the text field, push enter
2. Translation and pinyin appear below
3. F1 key to display help dialog
3. <Ctrl+B> on Linux or <Cmd+B> to display the about dialog
3. <Ctrl+S> on Linux or <Cmd+S> to save the phrase into the database
4. <Ctrl+F> on Linux or <Cmd+F> to search for saved phrases
5. <Ctrl+X> on Linux or <Cmd+X> to exercise (not implemented yet)
5. <Ctrl+,> on Linux or <Cmd+,> to open settings

![image](hanzi.png)
