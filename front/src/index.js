import { Elm } from "./Main.elm";

Elm.Main.init({
    node: document.getElementById("root"),
    flags: {
        supabaseKey: process.env.SUPABASE_KEY,
        supabaseUrl: process.env.SUPABASE_URL,
    },
});
