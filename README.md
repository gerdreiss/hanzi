# 汉字

Note: this project is a work in progress!

The goal of this project is to create a simple desktop app that queries a LLM model for translation and save it to a database, to be used later for learning.

## Usage

### Before using
1. Install [Ollama](https://ollama.com/)
2. `ollama run <model>` - take the model from the [model library](https://ollama.com/library). In my experience, mistral is the best LLM model for translations.

To see logs, start Hanzi providing environment variable RUST_LOG=\<log level\>

### How-to
1. Enter any chinese phrase into the text field, push enter
2. Pinyin and translation appear below
3. F1 key to display help (not implemented)
3. <Ctrl+A> on Linux or <Cmd+A> to display the about dialog  (not implemented)
3. <Ctrl+S> on Linux or <Cmd+S> to save the phrase into the database  (not implemented)
4. <Ctrl+F> on Linux or <Cmd+F> to search for saved phrases  (not implemented)
5. <Ctrl+,> on Linux or <Cmd+,> to open settings  (not implemented)

![image](hanzi.png)