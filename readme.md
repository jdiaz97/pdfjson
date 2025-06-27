# pdfy

AI-powered CLI tool to extract information from PDFs using a custom prompt.

## Usage

Make that you did this at some point

```
GROQ_API_KEY=your_api_key
```

And then

```
pdfy <pdf-file> "<your prompt>"
```

Example:

```
pdfy example.pdf "Make a summary of this paper, also get me all contact information."
```

## How to compile

```
cargo build --release
```