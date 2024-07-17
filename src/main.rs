#![allow(non_snake_case)]

mod arena;
mod bit_field;
mod button;
mod store;
mod world;

use arena::ArenaId;
use button::{Button, ButtonSize};
use dioxus::prelude::*;
use futures_util::StreamExt;
use idb::{DatabaseEvent, Factory, ObjectStoreParams, TransactionMode};
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use store::{Bookmark, Store};
use tracing::Level;
use wasm_bindgen::JsValue;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

enum Action {
    AddBookmark {
        title: String,
        link: String,
        note: String,
    },
}

#[component]
fn App() -> Element {
    let mut drawer_enabled = use_signal(|| true);
    let mut drawer_title = use_signal(|| String::new());
    let mut drawer_link = use_signal(|| String::new());
    let mut drawer_note = use_signal(|| String::new());

    // PERF: Don't ever read this
    let mut store = use_signal(move || Store::new());
    let mut cards = use_signal(move || Vec::with_capacity(0));

    let coroutine = use_coroutine(move |mut rx: UnboundedReceiver<Action>| async move {
        let serializer = Serializer::json_compatible();

        // Handle all the errors
        let factory = Factory::new().expect("Should be able to get factory");
        let mut indexed_db = factory
            .open("librarian", Some(1))
            .expect("should be able to open DB");

        indexed_db.on_upgrade_needed(|ev| {
            let database = ev.database().expect("DB should exist");

            let store_params = ObjectStoreParams::new();

            database
                .create_object_store("bookmarks", store_params)
                .expect("should be able to create store");
        });

        let indexed_db = indexed_db.await.expect("should be able to open DB");

        let transaction = indexed_db
            .transaction(&["bookmarks"], TransactionMode::ReadOnly)
            .expect("should be able to create transaction");
        let bookmarks_os = transaction
            .object_store("bookmarks")
            .expect("should be able to access object store");

        let entries = bookmarks_os
            .get_all(None, None)
            .expect("should be able to get all entries")
            .await
            .expect("should be able to get all entries");

        let mut store_access = store.write();

        for entry in entries {
            let bookmark: Bookmark =
                serde_wasm_bindgen::from_value(entry).expect("should be able to deserialize");
            store_access.add_bookmark(bookmark);
        }

        *cards.write() = store_access.all().collect();

        drop(transaction);
        drop(bookmarks_os);
        drop(store_access);

        while let Some(action) = rx.next().await {
            let mut store = store.write();

            match action {
                Action::AddBookmark { title, link, note } => {
                    store.create_bookmark(&title, &link, &note);
                }
            }
            let transaction = indexed_db
                .transaction(&["bookmarks"], TransactionMode::ReadWrite)
                .expect("should be able to create transaction");

            let bookmarks = transaction
                .object_store("bookmarks")
                .expect("should be able to access object store");

            for (id, bookmark) in store.changes() {
                let bookmark = bookmark
                    .serialize(&serializer)
                    .expect("should be able to serialize");
                let _ = bookmarks.put(&bookmark, Some(&JsValue::from_f64(id.id() as f64)));
            }

            *cards.write() = store.all().collect();
        }
    });

    let is_drawer_disabled = use_memo(move || {
        let title = || drawer_title().is_empty();
        let http = || drawer_link().starts_with("http://");
        let https = || drawer_link().starts_with("https://");

        title() || !(http() || https())
    });

    let onclick = move |_| {
        coroutine.send(Action::AddBookmark {
            title: drawer_title.cloned(),
            link: drawer_link.cloned(),
            note: drawer_note.cloned(),
        });

        drawer_title.set(String::new());
        drawer_link.set(String::new());
        drawer_note.set(String::new());
    };

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        link { rel: "stylesheet", href: "tailwind.css" }
        div {
            class: "min-h-full bg-secondary flex",
            if drawer_enabled() {
                div {
                    class: "bg-primary flex-1 border-r border-gray-200",
                    div {
                        class: "sticky top-0 p-8 flex flex-col gap-6 ",
                        h2 {
                            class: "text-3xl font-bold pb-4",
                            "New Bookmark"
                        }
                        input {
                            class: "bg-primary px-4 h-8 rounded-md border border-gray-300",
                            placeholder: "Title",
                            value: drawer_title,
                            oninput: move |ev| drawer_title.set(ev.value()),
                        }
                        input {
                            class: "bg-primary px-4 h-8 rounded-md border border-gray-300",
                            placeholder: "Link",
                            value: drawer_link,
                            oninput: move |ev| drawer_link.set(ev.value()),
                        }
                        textarea {
                            class: "bg-primary px-4 py-2 rounded-md border border-gray-300 resize-none",
                            rows: 16,
                            placeholder: "Note",
                            value: drawer_note,
                            oninput: move |ev| drawer_note.set(ev.value()),
                        }
                        Button {
                            value: "Add",
                            size: ButtonSize::Full,
                            disabled: is_drawer_disabled(),
                            onclick,
                        }
                    }
                }
            }
            div {
                class: "flex-[3] flex flex-col",
                div {
                    class: "sticky top-0 h-16 w-full bg-primary flex items-center border-b border-gray-200",
                    span {
                        class: "mx-8",
                        Button {
                            value: "New",
                            size: ButtonSize::Big,
                            onclick: move |_| drawer_enabled.set(!drawer_enabled()),
                        }
                    }
                    input {
                        class: "bg-primary flex-1 px-4 h-8 rounded-md border border-gray-300",
                        placeholder: "Search"
                    }
                }
                div {
                    class: "flex-1 w-full grid grid-cols-cards grid-rows-[min-content] p-8 gap-8",
                    for id in cards() {
                        Card {
                            store,
                            id,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Card(store: Signal<Store>, id: ArenaId<Bookmark>) -> Element {
    let bookmark = use_memo(move || store.peek().bookmark(id).clone());

    rsx! {
        div {
            class: "flex flex-col px-4 py-2 bg-primary shadow-md h-80 rounded-xl break-words",
            h3 {
                class: "h-16 text-sm font-bold",
                {bookmark().title}
            }
            div {
                a {
                    class: "text-sky-500 underline break-words",
                    href: "{bookmark().link}",
                    {bookmark().link}
                }
            }
            div {
                class: "flex-1 break-words",
                {bookmark().note}
            }
        }
    }
}
