import { Elm } from "./Main.elm";

Elm.Main.init({
    node: document.getElementById("root"),
    flags: {
        apiUrl: process.env.API_URL,
    },
});
