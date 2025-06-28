# pdfy

AI-powered CLI tool to extract information from PDFs using a custom prompt.

## Demo

<video src=https://github.com/user-attachments/assets/879c8d9c-3dcf-4342-9ba4-8165b41a1c6a>A video showing various Brush features and scenes</video>

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
