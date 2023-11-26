#import "@preview/codly:0.1.0": *
#set page(numbering: "1/1")
#set align(center)
#set text(font:"Monaspace Xenon")
#set text(size: 18pt,weight: "bold")
Calc Manual\
#set text(size: 13pt,weight: "regular")
Last updated, November, The 26th, 2023
#set align(left)
#show heading.where(level:1): set align(right)
#set heading(numbering: "I.1.")

#outline(title: [Table of Contents])
#pagebreak(weak:true)

#let icon(codepoint) = {
  box(
    height: 0.8em,
    baseline: 0.05em,
    image(codepoint)
  )
  h(0.1em)
}

#show: codly-init.with()
#codly(languages: (
  rust: (name: "Rust", icon: icon("brand-rust.svg"), color: rgb("#CE412B")),
  sh: (name: "Bash", icon: icon("brand-bash.svg"), color: rgb("3c3c3c")),
))

#let calc = link("https://calc.nwa2coco.fr",[#set text(red); *Calc*])

= Introduction
#v(1em)
#calc is a fully-featured calculator written in Rust for education purpose, it 
was designed to be minimalistic but then went off the rails and a lot of feature
where implemented.

Now #calc is a powerful calculator capable of exact rational computation,
matrix and vectors algebra, bindings to gnuplot and terminal plotting, with
dozens of updates and currently (as of writing this manual) in version *2.11.4*.

If you prefer a website you may want to read
#link("https://calc.nwa2coco.fr/book",[*The Online Book*]) which is always up to
date.

== Install

You can install it via cargo 

```sh
cargo install mini-calc
```

or via the source

```sh
git clone https://github.com/coco33920/calc
cd calc 
cargo build --release
./target/release/mini-calc
```

Visit #calc to see all the install page

== Contributors 


#table(
  columns: (auto, auto, auto),
  inset: 10pt,
  align: horizon,
  [*Name*], [*Role*], [*Website*],
  "Charlotte THOMAS",
  "Main developer/Maintener",
  link("https://me.nwa2coco.fr",[#set text(red); Personal Page]),
  "Léana 江",
  "Help, cleanup",
  link("https://earth2077.fr",[#set text(red); Website/Blog])
)
