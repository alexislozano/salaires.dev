const requestAnimationFrame =
    window.requestAnimationFrame ||
    window.mozRequestAnimationFrame ||
    window.webkitRequestAnimationFrame ||
    window.msRequestAnimationFrame;

import { Elm } from "./Main.elm";

const app = Elm.Main.init({
    node: document.getElementById("root"),
    flags: {
        apiUrl: process.env.API_URL,
        hCaptchaKey: process.env.HCAPTCHA_KEY,
        maintenance: process.env.MAINTENANCE === "true",
        noInsert: process.env.NO_INSERT === "true",
    },
});

app.ports.renderCaptcha.subscribe(function () {
    requestAnimationFrame(function () {
        hcaptcha.render("h-captcha", {
            callback: function (res) {
                app.ports.captchaReceived.send(res);
            },
        });
    });
});
