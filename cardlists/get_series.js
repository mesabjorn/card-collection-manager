
function filterRarity(text) {
    //replace short prints with common
    if (text.indexOf("Short Print") > -1) {
        return "Common";
    }
    if (text.indexOf("\n")>-1){
        text = text.split("\n")[0];
    }
    return text.trim();
}

function tableToJson(tableId) {
    const table = document.querySelectorAll("table.card-list")[0];
    const rows = Array.from(table.querySelectorAll("tbody tr"));

    return rows.map(row => {
        const cells = row.querySelectorAll("td");
        return {
            card_number: cells[0]?.innerText.trim() || "",
            name: cells[1]?.innerText.replace(/"/g, "").trim() || "",
            rarity: cells[2] ? filterRarity(cells[2].innerText) : "",
            category: cells[3]?.innerText.trim() || ""
        };
    });
}

function getReleaseDate() {
    const headers = [...document.querySelectorAll("aside>section>h2")];
    const release_date_index = headers.map(h => h.innerText.trim()).indexOf("Release dates");
    if (release_date_index === -1) {
        throw new Error("Couldn't parse release date");
    }

    dates = [...headers[release_date_index].parentElement.querySelectorAll(".pi-data-value")].map(v => v.innerText);
    if (dates.length === 0) {
        throw new Error("Couldn't parse release date");
    }
    return dates[0];
}


const toClipboard = async (text) => {
    if (navigator.clipboard && window.isSecureContext) {
        // Modern API
        await navigator.clipboard.writeText(text);
    } else {
        // Fallback
        const ta = document.createElement("textarea");
        ta.value = text;
        ta.style.position = "fixed"; // prevents scrolling to bottom
        ta.style.opacity = "0";
        document.body.appendChild(ta);
        ta.select();
        document.execCommand("copy");
        document.body.removeChild(ta);
    }
};


const main = () => {
    // Insert button into .main-container
    const container = document.querySelector(".main-container");
    if (container) {
        const btn = document.createElement("button");
        btn.innerText = "📋 Copy series Data";
        btn.style.fontSize = "24px";
        btn.style.padding = "16px 24px";
        btn.style.margin = "50px";
        btn.style.borderRadius = "12px";
        btn.style.cursor = "pointer";
        btn.style.background = "#4CAF50";
        btn.style.color = "white";
        btn.style.border = "none";
        btn.style.boxShadow = "0 4px 8px rgba(0,0,0,0.2)";

        // Click action
        btn.addEventListener("click", () => {

            const cards = tableToJson();//get card data
            //get metadata of set
            const jsonData = {
                "name": document.querySelector("#mw-content-text > div > aside > h2").innerText,
                "ncards": cards.length,
                "release_date": getReleaseDate(),
                "cards": cards
            }
            const text = JSON.stringify(jsonData, null, 2);
            toClipboard(text);
            btn.innerText = "✅ Copied!";
            setTimeout(() => (btn.innerText = "📋 Copy Data"), 1500);
        });

        // Use insertBefore to put button as the first element
        container.insertBefore(btn, container.firstChild);
    }
}

main();