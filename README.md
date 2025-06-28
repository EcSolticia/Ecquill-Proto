# Equill-Proto

 A simplistic and limited Markdown to HTML converter that I made to get started with learning Rust.

# Implemented Features

- H1 Headers.
- Regular text.
- Bold and italic text.

# Example Usage
The following is `test_input.md`:
```markdown
# Hello, and welcome.

This is my custom markdown-to-HTML converter. Have fun!

[This] is *supposed* to be a **link**, not yet fully implemented!

*bye!*
```
Generate HTML:
```bash
cargo run test_input.md output.html
```
Running the above command will generate the `output.html` file with the following contents:
```html
<html><body>
<h1> Hello, and welcome. </h1>
<p></p>
<p>This is my custom markdown-to-HTML converter. Have fun! </p>
<p></p>
<p><a>This </a>is <i>supposed </i>to be a <b>link </b>not yet fully implemented! </p>
<p></p>
<p><i>bye! </i></p>
<p></p>
</html></body>
```
This looks like the following:

![image](https://github.com/user-attachments/assets/b0d159f7-e80e-4849-9dce-476061c7677e)
