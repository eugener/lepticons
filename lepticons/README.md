# Leptions

Add super popular Lucide icons to your Leptos projects with ease. 
Every icon is packaged as it own Cargo feature, so you can import only the icons you need.

## Usage

```rust
<Icon glyph= move || LucideGlyph::Search/>
<Icon glyph=move || LucideGlyph::X 
      class="cursor-pointer" 
      on:click=clear_filter />
```

## Leptos Compatibility

| Leptos Version | Lepticons Version|
|----------------|------------------|
| 0.5.0          | 0.5.0            |

   
    