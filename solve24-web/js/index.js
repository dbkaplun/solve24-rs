import("../Cargo.toml")
    .catch(console.error)
    .then(() => { document.querySelector('body').classList.remove('loading'); })
    .catch(console.error);
