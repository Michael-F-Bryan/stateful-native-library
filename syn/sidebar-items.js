initSidebarItems({"enum":[["AttrStyle","Distinguishes between attributes that decorate an item and attributes that are contained within an item."],["BinOp","A binary operator: `+`, `+=`, `&`."],["Data","The storage of a struct, enum or union data structure."],["Expr","A Rust expression."],["Fields","Data stored within an enum variant or struct."],["GenericArgument","An individual generic argument, like `'a`, `T`, or `Item = T`."],["GenericParam","A generic type parameter, lifetime, or const generic: `T: Into<String>`, `'a: 'b`, `const LEN: usize`."],["Lit","A Rust literal such as a string or integer or boolean."],["MacroDelimiter","A grouping token that surrounds a macro body: `m!(...)` or `m!{...}` or `m![...]`."],["Member","A struct or tuple struct field accessed in a struct literal or field expression."],["Meta","Content of a compile-time structured attribute."],["NestedMeta","Element of a compile-time attribute list."],["PathArguments","Angle bracketed or parenthesized arguments of a path segment."],["ReturnType","Return type of a function signature."],["StrStyle","The style of a string literal, either plain quoted or a raw string like `r##\"data\"##`."],["TraitBoundModifier","A modifier on a trait bound, currently only used for the `?` in `?Sized`."],["Type","The possible types that a Rust value could have."],["TypeParamBound","A trait or lifetime used as a bound on a type parameter."],["UnOp","A unary operator: `*`, `!`, `-`."],["Visibility","The visibility level of an item: inherited or `pub` or `pub(restricted)`."],["WherePredicate","A single predicate in a `where` clause: `T: Deserialize<'de>`."]],"fn":[["parse","Parse tokens of source code into the chosen syntax tree node."],["parse2","Parse a proc-macro2 token stream into the chosen syntax tree node."],["parse_str","Parse a string of Rust code into the chosen syntax tree node."]],"macro":[["Token","A type-macro that expands to the name of the Rust type representation of a given token."],["braced","Parse a set of curly braces and expose their content to subsequent parsers."],["bracketed","Parse a set of square brackets and expose their content to subsequent parsers."],["custom_keyword","Define a type that supports parsing and printing a given identifier as if it were a keyword."],["custom_punctuation","Define a type that supports parsing and printing a multi-character symbol as if it were a punctuation token."],["parenthesized","Parse a set of parentheses and expose their content to subsequent parsers."],["parse_macro_input","Parse the input TokenStream of a macro, triggering a compile error if the tokens fail to parse."],["parse_quote","Quasi-quotation macro that accepts input like the `quote!` macro but uses type inference to figure out a return type for those tokens."]],"mod":[["buffer","A stably addressed token buffer supporting efficient traversal based on a cheaply copyable cursor."],["ext","Extension traits to provide parsing methods on foreign types."],["parse","Parsing interface for parsing a token stream into a syntax tree node."],["punctuated","A punctuated sequence of syntax tree nodes separated by punctuation."],["spanned","A trait that can provide the `Span` of the complete contents of a syntax tree node."],["token","Tokens representing Rust punctuation, keywords, and delimiters."]],"struct":[["Abi","The binary interface of a function: `extern \"C\"`."],["AngleBracketedGenericArguments","Angle bracketed arguments of a path segment: the `<K, V>` in `HashMap<K, V>`."],["Attribute","An attribute like `#[repr(transparent)]`."],["BareFnArg","An argument in a function type: the `usize` in `fn(usize) -> bool`."],["Binding","A binding (equality constraint) on an associated type: `Item = u8`."],["BoundLifetimes","A set of bound lifetimes: `for<'a, 'b, 'c>`."],["ConstParam","A const generic parameter: `const LENGTH: usize`."],["Constraint","An associated type bound: `Iterator<Item: Display>`."],["DataEnum","An enum input to a `proc_macro_derive` macro."],["DataStruct","A struct input to a `proc_macro_derive` macro."],["DataUnion","An untagged union input to a `proc_macro_derive` macro."],["DeriveInput","Data structure sent to a `proc_macro_derive` macro."],["Error","Error returned when a Syn parser cannot parse the input tokens."],["ExprArray","A slice literal expression: `[a, b, c, d]`."],["ExprAssign","An assignment expression: `a = compute()`."],["ExprAssignOp","A compound assignment expression: `counter += 1`."],["ExprAsync","An async block: `async { ... }`."],["ExprAwait","An await expression: `fut.await`."],["ExprBinary","A binary operation: `a + b`, `a * b`."],["ExprBlock","A blocked scope: `{ ... }`."],["ExprBox","A box expression: `box f`."],["ExprBreak","A `break`, with an optional label to break and an optional expression."],["ExprCall","A function call expression: `invoke(a, b)`."],["ExprCast","A cast expression: `foo as f64`."],["ExprClosure","A closure expression: `|a, b| a + b`."],["ExprContinue","A `continue`, with an optional label."],["ExprField","Access of a named struct field (`obj.k`) or unnamed tuple struct field (`obj.0`)."],["ExprForLoop","A for loop: `for pat in expr { ... }`."],["ExprGroup","An expression contained within invisible delimiters."],["ExprIf","An `if` expression with an optional `else` block: `if expr { ... } else { ... }`."],["ExprIndex","A square bracketed indexing expression: `vector[2]`."],["ExprLet","A `let` guard: `let Some(x) = opt`."],["ExprLit","A literal in place of an expression: `1`, `\"foo\"`."],["ExprLoop","Conditionless loop: `loop { ... }`."],["ExprMacro","A macro invocation expression: `format!(\"{}\", q)`."],["ExprMatch","A `match` expression: `match n { Some(n) => {}, None => {} }`."],["ExprMethodCall","A method call expression: `x.foo::<T>(a, b)`."],["ExprParen","A parenthesized expression: `(a + b)`."],["ExprPath","A path like `std::mem::replace` possibly containing generic parameters and a qualified self-type."],["ExprRange","A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`."],["ExprReference","A referencing operation: `&a` or `&mut a`."],["ExprRepeat","An array literal constructed from one repeated element: `[0u8; N]`."],["ExprReturn","A `return`, with an optional value to be returned."],["ExprStruct","A struct literal expression: `Point { x: 1, y: 1 }`."],["ExprTry","A try-expression: `expr?`."],["ExprTryBlock","A try block: `try { ... }`."],["ExprTuple","A tuple expression: `(a, b, c, d)`."],["ExprType","A type ascription expression: `foo: f64`."],["ExprUnary","A unary operation: `!x`, `*x`."],["ExprUnsafe","An unsafe block: `unsafe { ... }`."],["ExprWhile","A while loop: `while expr { ... }`."],["ExprYield","A yield expression: `yield expr`."],["Field","A field of a struct or enum variant."],["FieldsNamed","Named fields of a struct or struct variant such as `Point { x: f64, y: f64 }`."],["FieldsUnnamed","Unnamed fields of a tuple struct or tuple variant such as `Some(T)`."],["Generics","Lifetimes and type parameters attached to a declaration of a function, enum, trait, etc."],["Ident",""],["ImplGenerics","Returned by `Generics::split_for_impl`."],["Index","The index of an unnamed tuple struct field."],["Lifetime","A Rust lifetime: `'a`."],["LifetimeDef","A lifetime definition: `'a: 'b + 'c + 'd`."],["LitBool","A boolean literal: `true` or `false`."],["LitByte","A byte literal: `b'f'`."],["LitByteStr","A byte string literal: `b\"foo\"`."],["LitChar","A character literal: `'a'`."],["LitFloat","A floating point literal: `1f64` or `1.0e10f64`."],["LitInt","An integer literal: `1` or `1u16`."],["LitStr","A UTF-8 string literal: `\"foo\"`."],["Macro","A macro invocation: `println!(\"{}\", mac)`."],["MetaList","A structured list within an attribute, like `derive(Copy, Clone)`."],["MetaNameValue","A name-value pair within an attribute, like `feature = \"nightly\"`."],["ParenthesizedGenericArguments","Arguments of a function path segment: the `(A, B) -> C` in `Fn(A,B) -> C`."],["Path","A path at which a named item is exported: `std::collections::HashMap`."],["PathSegment","A segment of a path together with any path arguments on that segment."],["PredicateEq","An equality predicate in a `where` clause (unsupported)."],["PredicateLifetime","A lifetime predicate in a `where` clause: `'a: 'b + 'c`."],["PredicateType","A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`."],["QSelf","The explicit Self type in a qualified path: the `T` in `<T as Display>::fmt`."],["TraitBound","A trait used as a bound on a type parameter."],["Turbofish","Returned by `TypeGenerics::as_turbofish`."],["TypeArray","A fixed size array type: `[T; n]`."],["TypeBareFn","A bare function type: `fn(usize) -> bool`."],["TypeGenerics","Returned by `Generics::split_for_impl`."],["TypeGroup","A type contained within invisible delimiters."],["TypeImplTrait","An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or a lifetime."],["TypeInfer","Indication that a type should be inferred by the compiler: `_`."],["TypeMacro","A macro in the type position."],["TypeNever","The never type: `!`."],["TypeParam","A generic type parameter: `T: Into<String>`."],["TypeParen","A parenthesized type equivalent to the inner type."],["TypePath","A path like `std::slice::Iter`, optionally qualified with a self-type as in `<Vec<T> as SomeTrait>::Associated`."],["TypePtr","A raw pointer type: `*const T` or `*mut T`."],["TypeReference","A reference type: `&'a T` or `&'a mut T`."],["TypeSlice","A dynamically sized slice type: `[T]`."],["TypeTraitObject","A trait object type `Bound1 + Bound2 + Bound3` where `Bound` is a trait or a lifetime."],["TypeTuple","A tuple type: `(A, B, C, String)`."],["Variadic","The variadic argument of a foreign function."],["Variant","An enum variant."],["VisCrate","A crate-level visibility: `crate`."],["VisPublic","A public visibility level: `pub`."],["VisRestricted","A visibility level restricted to some path: `pub(self)` or `pub(super)` or `pub(crate)` or `pub(in some::module)`."],["WhereClause","A `where` clause in a definition: `where T: Deserialize<'de>, D: 'static`."]],"type":[["AttributeArgs","Conventional argument type associated with an invocation of an attribute macro."],["Result","The result of a Syn parser."]]});