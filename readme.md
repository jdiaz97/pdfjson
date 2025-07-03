# pdfx

AI-powered CLI tool to extract information from PDFs using a custom prompt.

## Demo

<video src=https://github.com/user-attachments/assets/ff53181b-07d2-452a-8d15-898634447f13>A video showing various Brush features and scenes</video>

## Usage

You need this environment variable by exporting it in your terminal 

```
GROQ_API_KEY=your_api_key
```

OR

```
OPENAI_API_KEY=your_api_key
```



And then you need  to compile the application using the command for compilation below



## How to compile

```
cargo build --release
```

You should be aware that When you do a plain `cargo build --release`, the `pdfx` binary ends up in your project’s `target/release/` folder and it means you haven’t installed it into your PATH yet. So  You have three easy ways to test this pdfx Application:

---

### 1. Run it directly from `target/release/`

```bash
./target/release/pdfx AcceptablePolicy.pdf "Make a summary of this paper, also get me all contact information." --openai
```  


### 



This assumes that you intend to use the openaikey after you have  done

```bash
export OPENAI_API_KEY=your_groq_key
```
---
in  your terminal if you plan to use OPENAIAPIKEY   or else


```bash
export GROK_API_KEY=your_groq_key
```

```bash
./target/release/pdfx AcceptablePolicy.pdf "Make a summary of this paper, also get me all contact information." 
```   

to test or  use GROK_API_KEY  without using any flag (--)


### 2. Use `cargo run`

```bash
cargo run --release -- example.pdf "Make a summary of this paper, also get me all contact information." --openai
```

or 


```bash
cargo run --release -- example.pdf "Make a summary of this paper, also get me all contact information." 
```

---

### 3. Install it into your Cargo bin (so `pdfx` is on your PATH)

```bash
cargo install --path .
```

That compiles and copies the `pdfx` binary into `~/.cargo/bin`, which is usually in your PATH. After that you can just do:

```bash
pdfx example.pdf "Make a summary of this paper, also get me all contact information." --openai
```


Example using Grok:

```
pdfx example.pdf "Make a summary of this paper, also get me all contact information."
```

Example using Openai:

```
cargo run --release -- AcceptablePolicy.pdf "Make a summary of this paper, also get me all contact information." --openai

```