[Project](README.md) | [References](references.md) | [Timeline](updates.md)

# References

A list of useful resources, links, and solutions utilized in the project. While the links provided are crucial i am missing some tutorials, stackoverflows and libraries that helped in the project but does not standout in the development of the project as a whole i have included stackoverflows that solvd crutial bugs.  
## Coinmarketcap
- **Link:** [coinmarketcap.com](https://coinmarketcap.com)
- **Description:** Provides cryptocurrency information such as prices, trade volumes, and market capitalization.
- **Usage:** Used for data in an SSE stream with no CORS.

## Rocket
- **Link:** [rocket.rs](https://rocket.rs)
- **Description:** A web framework for Rust, simplifying the creation of fast, type-safe, secure web applications.
- **Usage:** Used for the web server aspect. Referenced documentation and boilerplates on their [GitHub](https://github.com/SergioBenitez/Rocket) and [website](https://rocket.rs).

## Rocket SSE
- **Link:** [Documentation on SSE Streams for Rocket](https://api.rocket.rs/master/rocket/response/stream/struct.EventStream.html)
- **Description:** Reference documentation for implementing SSE streams in Rocket.
- **Usage:** Helpful reference for implementing SSE in the project.

## vue.draggable.next
- **Link:** [github.com/SortableJS/vue.draggable.next](https://github.com/SortableJS/vue.draggable.next)
- **Description:** Vue.js 3.0 component enabling drag-and-drop and synchronization with the view model array.
- **Usage:** Enabled draggable coins on the Vue frontend.

## Tailwind
- **Link:** [tailwindcss.com](https://tailwindcss.com/)
- **Description:** Rapidly builds modern websites directly in HTML.
- **Usage:** Replaced the majority of frontend CSS; provides professional designs with flexibility.

## Vite
- **Link:** [vitejs.dev](https://vitejs.dev)
- **Description:** Next-generation frontend tooling.
- **Usage:** Used for the frontend boilerplate.

## Transform Tools
- **Link:** [JSON to Rust Serde Structs](https://transform.tools/json-to-rust-serde)
- **Description:** Tool for transforming JSON from CMC (CoinMarketCap) into Rust Serde Structs.
- **Usage:** Used for handling CMC JSON data.

## CORS Rocket.rs
- **Link:** [How to set up CORS for Rocket.rs](https://stackoverflow.com/questions/62412361/how-to-set-up-cors-or-options-for-rocket-rs)
- **Description:** Solution for CORS with Rocket provided by "Ibraheem Ahmed".
- **Usage:** Used to enable CORS for communication with the frontend.

## Lazy Static
- **Link:** [Lazy Static Documentation](https://docs.rs/lazy_static/latest/lazy_static/)
- **Description:** Macro for initializing statics requiring runtime code execution.
- **Usage:** Used for interacting with stored coins across different tasks/threads/instances.

## Tokio
- **Link:** [Tokio GitHub](https://github.com/tokio-rs/tokio), [Tokio Docs](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html)
- **Description:** Runtime for writing reliable asynchronous applications with Rust.
- **Usage:** Assisted in creating new threads/tasks and implementing the Arc Mutex solution for interacting with scraped data.
