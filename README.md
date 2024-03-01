```
cargo install create-leptos-csr-tw
create-leptos-csr-tw
```

- [x] TailwindCSS for styling
- [x] Serves image content from `/public` that trunk recognizes
- [x] Mobile viewport configuration 
- [x] [`Leptos-use`](https://github.com/Synphonyte/leptos-use), a collection of Leptos utilities

```
cargo add leptos --features=csr,nightly
npx tailwindcss init
trunk serve --open
```
