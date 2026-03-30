# QuickPaste
---

QuickPaste is a rocket-based paste/haste-bin clone written in Rust, using <a href="https://rocket.rs">rocket</a>.

---

## Usage

Clone the repository, cd into it, and run "cargo run". Either click the link in your terminal or go to 127.0.0.1:8000/

---

## FAQs

### What differs QuickPaste from the "pastebin" rocket example?

QuickPaste started off from me working through that example in the rocket docs, and has been extended upon with the following features:
- Only accepts pastes from the webform on / (More Secure?)
- Automatic code syntax highlighting on the paste display page!
- Randomly generated English word group paste names. (Memorable paste links!)
- Tera for templating and pico-css for styling! (No more plaintext usage pages!)

---

