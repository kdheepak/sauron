## General

 - [X] Add a prelude to import all html tags, svg tags, html attributes, svg attributes that doesn't conflicts
 - [X] Make a module that isolate the `with-dom` features.
 - [X] Rework the dumb patch taking the advantage of feature gating the dom capability.
 - [ ] Add documentation to functions
 - [ ] Loosen the lifetime requirement of the `Fn` inside `Callback` from `'static` to a generic one eg: `'c`
     - [X] Done in `mt-dom` branch: non-static-lifetime
 - [X] Deprecate the tag macro since it complicates the conflict in reexporting the functions
     - ie: `style!` as a tag, `style!` macro for attributes, `style` as attribute call.
 - [X] Change the README example to use the node macro syntax
     - rename the old `minimal` to `minimal-alt` and use the `node-macro-syntax` in `minimal` example
 - [X] Move `sauron-syntax` into `html2sauron` project
 - [ ] Expose Cmd,Component outside of `with-dom` feature gate
     - This would allow a total isomorphic app reusing the components
     - [ ] Make an equivalent for Program(client-side updater) for use in server-side
        - ie: ServerRender, where Msg could be passed as a data to hydrate the view (template) before sending to the client
 - [X] Fix the render function where attributes of the same name not merged
 - [X] Change type of tag, attribute_name, style keys from `&'static str` to `&'a str`
     - This will remove the need for hardcode HTML_STYLES lookup, which could be a performance penalty
 - [ ] Add the RealWorld example
     - Use the elm base code https://github.com/rtfeldman/elm-spa-example
 - [ ] **breaking** ~put back `style` as a normal attribute~, to avoid possible confusion to new users.
     - Cancelled, since style is treated differently in attributes.
 - [X] **breaking** merge `Browser` to `Window`.
 - [ ] Add `and_then`, `sequence` to `Cmd` to perform a task after the preceding Cmd succeeds.
 - [ ] Create a document on why it is needed for events such as `on_click` to have a copy of the variables outside of its environment.

## Internal
- [ ] Find a way to map `Cmd<APP,MSG>` to `Cmd<APP2, MSG2>`
        ie: `Cmd<ChildApp, ChildMsg>` to `Cmd<App, Msg>`
        This is needed since `Cmd` from `update` function of sub components
        are not dispatched in the program. Only the top level
        component `Cmd` can be dispatched
    - [ ] Find a way to map `Program<APP,MSG>` to `Program<APP2,MSG2>`
        - [X] map `DomUpdater<MSG>` to `DomUpdater<MSG2>`
        - Issue mapping fields of Program that are in `Rc<RefCell>` seems not that simple
            as the Rc value of dom_updater is to be borrowed and will have a borrow checker issue
- [ ] Merge `Program` and `DomUpdater`
    - Issue: DomUpdater has multiple fields, which would then be wrap with `Rc<RefCell>` individually
- [ ] Change the `'static` of trait implementation by specifying the lifetime
        - ref: https://stackoverflow.com/questions/52187644/lifetime-must-be-valid-for-the-static-lifetime-so-that-the-types-are-compatible
- [X] Get rid of test_fixtures and move it to test directory
- [ ] Make each component have a reference to the root dom where it is mounted.
    - This will make local state changes to the component easier to do, as opposed to diffing the whole DOM tree.
- [X] Unify the code of Program replace_mount, append_mount
- [ ] replace the request_animation_frame with the code from execute_request_animation frame
- [ ] Create a function to derive Component name from the struct name of the Component
    and preprocess the jss with it before injecting it to the main program
- [X] Clean up `CreateNode`
    - no need to wrap `Node` and `Element` instead just return them as created with their `closures`
- [X] Cmd should include a `should_update: bool` field which indicates if the update should be made or not
        - Cmd{ commands:Vec<..>,should_update }
        - Cmd::noop() // no update operation
        - Fixed in `0.36.0`
- [ ] Remove the Dispatch trait and pass Program as it is in dom_updater and apply_patches module
    - There is only one implementation of `Dispatch` trait anyway, that is `Program`
- [X] ISSUE: sauron `node!` macro doesn't work on svg tags since it is using only `html_element` function which `namespace` is not supplied.
    - Fixed in `0.35.0` by checking whether a tag has a namespace.
- [ ] Change `program: Option<&DSP>` to just `program: &DSP` since there program is needed everywhere.
- [X] Improve `sauron-node-macro` to call on the equivalent html function instead of `elment_ns`.
    - This would improve performance since the function already has the information whether or not it has namespace or not.
    - Mitigated with the use of `Lazy` `HashSet` look up in `sauron-parse` for faster lookup.
    - Further improved using `sauron-parse` by resolving the value of `self-closing` and `namespace` at compile time in the `node` macro.
- [X] Create a hashed collections in `sauron-parse` to optimize lookup of tags for namespace or self-closing
    - Created a fast lookup for `is_self_closing(tag)` amd `tag_namespace`.
- [ ] old elements that has an event attached has no way of knowing the equivalent new element has the same event attached as their callbacks are clone of closures inside of Rc
    and no 2 closures are the same even if the have the same code.



## Features
- [X] Storage service (May not be needed since the user can directly use web-sys)
    - [X] using wasm-bindgen directly will remove the need for Storage service wrapper
- [X] Fetch service
- [X] Url change service
    - using wasm-bindgen directly eliminates the need for Url change service wrapper
- [ ] re-think about the `sauron-core` features:
    - [X] `with-dom` when used in client-side, default
    - [ ] `with-ssr` when used in server-side rendering, mutually exlusive to `with-dom`
    - [X] `no_request_animation_frame` this should be additive
        -  crate is now using `with-request-animation` feature
- [X] `with-markdown`
    - [X] Add sanitation to markdown parser, use `ammonia` crate
    - [X] expose the `sauron-md` as `sauron::markdown` module, behind a feature flag
- [X] Add example using markdown
- [X] Make use of `serde_json` to parse `style` into components


## Performance
- [X] Fix the reported issues with benchmarks
    - fixed by setting the target to web when building the wasm
- [ ] Create a new benchmark for the js-comprehensive-benchmark suite
    - [link](https://github.com/krausest/js-framework-benchmark)
    - [ ] Initial attempt https://github.com/ivanceras/performance-test-sauron
- [ ] Use Weak pointer for program instead of Rc where strong reference is not needed.

## Maintenance
- [ ] Move `sauron-markdown` into it's own repo, for keeping sauron slim.
- [ ] Move `jss` into a new crate `sauron-jss` for keeping sauron slim.
    - [ ] Use [json](https://github.com/maciejhirsz/json-rust) crate for `jss`.
        - The quote on keys are optional, so this is good for use in writing css.
