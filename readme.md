# pdfx

AI-powered CLI tool to extract information from PDFs using a custom prompt.

## Demo

<video src=https://github.com/user-attachments/assets/ff53181b-07d2-452a-8d15-898634447f13>A video showing various Brush features and scenes</video>

## Usage

First

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
