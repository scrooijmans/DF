Title: Command Line | Graphviz

Description: DOT rendering programs and utilities.

View page source Edit this page Create child page Create documentation issue Print entire section

# Command Line

DOT rendering programs and utilities.

All Graphviz programs have a similar invocation:

_cmd_ \[ _flags_ \] \[ _input files_ \]

For example:

```
$ dot -Tsvg input.dot
```

If no input files are supplied, the program reads from **stdin**. For example:

```
$ echo 'digraph { a -> b }' | dot -Tsvg > output.svg
```

Generates:

## Flags

### `-G`_name_\[=_value_\]

Set a graph attribute, with default _value_ = `true`

For example,

```
$ echo 'digraph { a -> b }' | dot -Tsvg -Gfontcolor=red -Glabel="My favorite letters"
```

Overrides the default `fontcolor` and `label` attributes of the graph, producing a red legend:

### `-N`_name_\[=_value_\]

Set a default node attribute, with default _value_ = `true`.

For example,

```
$ echo 'digraph { a -> b }' | dot -Tsvg -Nfontcolor=red -Nshape=rect
```

Overrides the default node `fontcolor` and `shape` attributes, producing rectangular nodes with red text:

### `-E`_name_\[=_value_\]

Set a default edge attribute, with default _value_ = `true`.

For example,

```
$ echo 'digraph { a -> b }' | dot -Tsvg -Ecolor=red -Earrowhead=diamond
```

Overrides the default edge `color` and `arrowhead` attributes, producing red edges with a diamond arrowhead:

### `-K`_layout_

Specifies which default layout engine to use, overriding the default from the command name. For example, running `dot -Kneato` is equivalent to running `neato`.

### `-T`_format_\[:_renderer_\[:_formatter_\]\]

Set output language to one of the supported formats. By default, attributed dot is produced.

Depending on how Graphviz was built, there may be multiple renderers for generating a particular output format, and multiple formatters for creating the final output. For example, a typical installation can produce PNG output using either the Cairo or GD library. The desired rendering engine can be specified after a colon. If there are multiple formatting engines available, the desired one can be specified in a similar fashion after the rendering engine. Thus, `-Tpng:cairo` specifies PNG output produced by Cairo (using the Cairo's default formatter), and `-Tpng:cairo:gd` specifies PNG output produced by Cairo formatted using the GD library.

If no renderer is specified, or a renderer but no formatter, the default one is invoked. The flag `-T_format_:` produces a list of all of the renderers available for the specified _format_, the first one listed with a prefix matching _format_ being the default. Using the `-v` flag, described below, will print which format, renderer, and formatter are actually used.

### `-V`

Emit version information and exit. For example:

```
$ dot -V
dot - graphviz version 2.47.1 (20210417.1919)
```

### `-l`_library_

User-supplied, device-dependent library text. Multiple flags may be given. These strings are passed to the code generator at the beginning of output.

For PostScript output, they are treated as file names whose content will be included in the preamble after the standard preamble. If _library_ is the empty string `""`, the standard preamble is not emitted.

### `-n`\[_num_\]

Sets no-op flag in **neato**. If set, **neato** assumes nodes have already been positioned and all nodes have a pos attribute giving the positions. It then performs an optional adjustment to remove node-node overlap, depending on the value of the overlap attribute, computes the edge layouts, depending on the value of the splines attribute, and emits the graph in the appropriate format. If _num_ is supplied, the following actions occur:

_num_ = 1

Equivalent to `-n`.

_num_ > 1

Use node positions as specified, with no adjustment to remove node-node overlaps, and use any edge layouts already specified by the pos attribute. **neato** computes an edge layout for any edge that does not have a **pos** attribute. As usual, edge layout is guided by the splines attribute.

### `-o`_outfile_

Write output to file _outfile_. For example,

```
$ echo 'digraph { a -> b }' | dot -Tsvg -o output.svg
```

Generates `output.svg`:

By default, output goes to **stdout**.

### `-O`

Automatically generate output file names based on the input file name and the various output formats specified by the **\-T** flags.

For example,

```
$ dot -Tsvg -O ~/family.dot ~/debug.dot
```

Generates `~/family.dot.svg` and `~/debug.dot.svg` files.

### `-P`

Automatically generate a graph that shows the plugin configuration of the current executable. e.g.

```
$ dot -P -Tsvg -o plugins.svg
```

### `-q`

Suppress warning messages.

### `-s`\[_scale_\]

Set input scale to _scale_. If this value is omitted, 72.0 is used. This number is used to convert the point coordinate units used in the pos attribute into inches, which is what is expected by neato and fdp. Thus, feeding the output of a graph laid out by one program into neato or fdp almost always requires this flag. Ignored if the `-n` flag is used.

### `-v`

Verbose mode

### `-x`

In **neato**, on input, prune isolated nodes and peninsulas. This removes uninteresting graph structure and produces a less cluttered drawing.

### `-y`

By default, the coordinate system used in generic output formats, such as attributed dot, extended dot, plain and plain-ext, is the standard cartesian system with the origin in the lower left corner, and with increasing y coordinates as points move from bottom to top. If the `-y` flag is used, the coordinate system is inverted, so that increasing values of y correspond to movement from top to bottom.

### `-?`

Print usage information, then exit.

If multiple `-T` flags are given, drawings of the graph are emitted in each of the specified formats. Multiple `-o` flags can be used to specify the output file for each format. If there are more formats than files, the remaining formats are written to **stdout**.

Note that the `-G`, `-N` and `-E` flags override any initial attribute declarations in the input graph, i.e., those attribute statements appearing before any node, edge or subgraph definitions. In addition, these flags cause the related attributes to be permanently attached to the graph. Thus, if attributed dot is used for output, the graph will have these attributes.

## Environment Variables

### `GDFONTPATH`

List of pathnames giving directories which a program should search for fonts. Overridden by DOTFONTPATH. _Used only if Graphviz is not built with the `fontconfig` library_

### `DOTFONTPATH`

List of pathnames giving directories which a program should search for fonts. Overridden by **fontpath**. _Used only if Graphviz is not built with the `fontconfig` library_

### `SERVER_NAME`

If defined, this indicates that the software is running as a web application, which restricts access to image files.

### `GVBINDIR`

Indicates which directory contains the Graphviz config file and plug-in libraries. If it is defined, the value overrides any other mechanism for finding this directory. If Graphviz is properly installed, it should not be needed, though it can be useful for relocation on platforms not running Linux or Windows.

##### acyclic

Make directed graphs acyclic.

##### bcomps

Biconnected components filter for graphs.

##### ccomps

Connected components filter for graphs.

##### cluster

Find clusters in a graph and augment the graph with this information.

##### diffimg

Calculates intersection between two images.

##### dijkstra

Single-source distance filter.

##### dotty

A customizable graph editor.

##### edgepaint

Edge coloring to disambiguate crossing edges.

##### gc

Count graph components.

##### gml2gv

GML-DOT converters.

##### graphml2gv

GRAPHML-DOT converter.

##### gv2gxl

GXL-GV converters.

##### gvcolor

Flow colors through a ranked digraph.

##### gvedit

Simple graph editor and viewer.

##### gvgen

Generate graphs.

##### gvmap

Find clusters and create a geographical map highlighting clusters.

##### gvpack

Merge and pack disjoint graphs.

##### gvpr

Graph pattern scanning and processing language.

##### gxl2gv

GXL-GV converters.

##### lefty

A programmable graphics editor.

##### lneato

A customizable graph editor.

##### mingle

Fast edge bundling.

##### mm2gv

Matrix Market-DOT converters.

##### nop

Pretty-print graph file.

##### sccmap

Extract strongly connected components of directed graphs.

##### smyrna

Interactive graph viewer.

##### tred

Transitive reduction filter for directed graphs.

##### unflatten

Adjust directed graphs to improve layout aspect ratio.

##### vimdot

Combined text editor and dot viewer.

Last modified July 28, 2024: Replace all Hugo 'ref's with 'relref's (bbef86a)
