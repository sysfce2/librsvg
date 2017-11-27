Librsvg
=======

This is librsvg - A small library to render Scalable Vector Graphics
([SVG][svg]), associated with the [GNOME Project][gnome].  It renders
SVG files to [Cairo][cairo] surfaces.  Cairo is the 2D, antialiased
drawing library that GNOME uses to draw things to the screen or to
generate output for printing.

Do you want to render non-animated SVGs to a Cairo surface with a
minimal API?  Librsvg may be adequate for you.

Using librsvg
-------------

**Documentation:** You can read the [documentation for librsvg][docs] at
developer.gnome.org.  Please [tell us][mail] if you don't find
something there that you need.

**Bug tracking:** If you have found a bug, take a look at [our bug
tracker][bugs].  Please see the "[reporting bugs][reporting-bugs]"
section in the file [CONTRIBUTING.md][contributing] to see how to
provide a good bug report.

**Asking questions:** Feel free to ask questions about using librsvg
in the [desktop-devel-list][d-d-l] mailing list.

**Programming languages:** Librsvg exports its API through [GObject
Introspection][gi].  This way, it is available in many programming
languages other than C.  Please see your language binding's
documentation for information on how to load the `Rsvg` namespace.

Contributing to librsvg's development
-------------------------------------

There is a code of conduct for contributors to librsvg; please see the
file [`code_of_conduct.md`][coc].

For information on how to report bugs, or how to contribute to librsvg
in general, please see the file [`CONTRIBUTING.md`][contributing].

Goals of librsvg
----------------

Librsvg aims to be a low-footprint library for rendering SVG images.
It is used primarily in the [GNOME project](https://www.gnome.org) to
render SVG icons and vector images that appear on the desktop.  It is
also used in Wikimedia to render the SVG images that appear in
Wikipedia, so that even old web browsers can display them.  Many
projects which casually need to render static SVG images use librsvg.

We aim to be a "render this SVG for me, quickly, and with a minimal
API" kind of library.

Feature additions will be considered on a case-by-case basis.

Non-goals of librsvg
--------------------

We don't aim to:

* Implement every single SVG feature that is in the spec.

* Implement scripting or external access to the SVG's DOM.

* Implement support for CSS-based animations (but if you can think of
  a nice API to do this, we would be glad to know!)

* Replace the industrial-strength SVG rendering machinery in modern
  web browsers.

Of course, [contributions are welcome][contributing].  In particular,
if you find nice ways of doing the above while still maintaining the
existing API of librsvg, we would love to know about it!

Maintainers
-----------

The maintainer of librsvg is [Federico Mena Quintero][federico].  You
can [mail me][mail] for any other questions you have about librsvg.

[svg]: https://en.wikipedia.org/wiki/Scalable_Vector_Graphics
[gnome]: https://www.gnome.org/
[cairo]: https://www.cairographics.org/
[coc]: code-of-conduct.md
[docs]: https://developer.gnome.org/rsvg/stable/
[mail]: mailto:federico@gnome.org
[bugs]: https://bugzilla.gnome.org/page.cgi?id=browse.html&product=librsvg
[gi]: https://wiki.gnome.org/Projects/GObjectIntrospection
[contributing]: CONTRIBUTING.md
[reporting-bugs]: CONTRIBUTING.md#reporting-bugs
[d-d-l]: https://mail.gnome.org/mailman/listinfo/desktop-devel-list
[federico]: https://people.gnome.org/~federico/
