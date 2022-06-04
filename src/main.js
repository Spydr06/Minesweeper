import init, { 
    getState, 
    openField, 
    toggleFlag 
} from "../pkg/minesweeper.js";

async function main() {
    await init();
    render();
}

function render() {
    let root = document.getElementById("root");
    let data = getState().split("\n").map(row => row.trim().split(/\s+/));

    root.innerHTML = "";
    root.style.display = "inline-grid";
    root.style.gridTemplate = `repeat(${data.length}, auto) / repeat(${data[0].length}, auto)`;

    for(let y = 0; y < data.length; y++) {
        for(let x = 0; x < data[y].length; x++) {
            let element = document.createElement("a");
            element.classList.add("field");
            element.href = "#";
            element.innerText = data[y][x];

            element.addEventListener("click", evt => {
                evt.preventDefault();
                openField(x, y);
                render();
            });

            element.addEventListener("contextmenu", evt => {
                evt.preventDefault();
                toggleFlag(x, y);
                render();
            })

            root.appendChild(element);
        }
    }
}

main();