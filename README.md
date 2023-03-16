# OpenAI Rust API
A manual implementation of the OpenAI API based on its 
[docs](https://platform.openai.com/docs/api-reference/introduction) in pure Rust.

## Current Status
- [x] Authentication
- [x] Models
- [x] Completions
- [ ] Chat
- [ ] Edits
- [ ] Images
- [ ] Embeddings
- [ ] Audio
- [ ] Files
- [ ] Fine-tunes
- [ ] Moderations
- [ ] Engines

## Plans
- Implement all of functionality
- Refactor code: 
  - Maybe try to improve the security of storing the api key in memory by using cryptography, not sure 
  - Readability
  - Performance
  - Type security (have some ideas for using nutype for specific values and maybe subtyping for requests)
  - Error handling
  - Docs
- Add tests
- Better examples (potentially with gui)