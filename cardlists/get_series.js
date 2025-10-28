function filterRarity(text) {
  //replace short prints with common

  if (text.indexOf("Short Print") > -1) {
    return ["Common"];
  }

  let texts = text.split("\n");

  return texts.map((t) => t.trim());
}

String.prototype.capitalize = function () {
  //capitalize each word in string
  return this.split(" ")
    .map((word) => {
      return word.slice(0, 1).toUpperCase() + word.slice(1);
    })
    .join(" ");
};

const getCardsFromTable = (table) => {
  const tbody = table.querySelector("tbody");
  const thead = table.querySelector("thead");

  if (!thead || !tbody) {
    console.error("Table has no header or body");
    return null;
  }

  const headings = [...thead.querySelectorAll("th")].map((e) =>
    e.innerText.trim()
  );
  if (headings.includes("English Name")) {
    console.error("Skipping non-english table");
    return null;
  }

  if (!headings.includes("Card number")) {
    console.error("Card number heading not found in table");
    return null;
  }

  if (!headings.includes("Name")) {
    console.error("Card Name heading not found in table");
    return null;
  }

  const rows = tbody.children;
  let cards = [];

  for (let r of rows) {
    const cells = r.querySelectorAll("td");
    const rarities = cells[2] ? filterRarity(cells[2].innerText) : "";
    for (let rarity of rarities) {
      const card = {
        card_number: cells[0]?.innerText.trim() || "",
        name: cells[1]?.innerText.replace(/"/g, "").trim() || "",
        rarity: rarity,
        category: cells[3]?.innerText.trim().capitalize() || "",
      };
      cards.push(card);
    }
  }
  return cards;
};

function tableToJson() {
  let tables = document.querySelectorAll("table"); //default:query all card tables
  let current = document.querySelectorAll(
    "div.wds-tab__content.wds-is-current"
  )[0];
  if (current) {
    //when page has a language selector, use the selected table 'current'
    tables = current.querySelectorAll("table.card-list");
  }

  let allCards = [];
  for (let t of tables) {
    let cards = getCardsFromTable(t);
    if (cards) {
      allCards = [...allCards, ...cards];
    }
  }
  return allCards;
}

function getReleaseDate() {
  const headers = [...document.querySelectorAll("aside>section>h2")];
  const release_date_index = headers
    .map((h) => h.innerText.trim())
    .indexOf("Release dates");
  if (release_date_index === -1) {
    throw new Error("Couldn't parse release date");
  }

  dates = [
    ...headers[release_date_index].parentElement.querySelectorAll(
      ".pi-data-value"
    ),
  ].map((v) => v.innerText);
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
    btn.style.position = "fixed";
    btn.style.width = "60%";
    btn.style.zIndex = 99;

    // Click action
    btn.addEventListener("click", () => {
      const cards = tableToJson(); //get card data
      //get metadata of set
      const jsonData = {
        name: document.querySelector("#mw-content-text > div > aside > h2")
          .innerText,
        ncards: cards.length,
        release_date: getReleaseDate(),
        cards: cards,
        prefix: cards[0].card_number.split("-")[0],
      };
      const text = JSON.stringify(jsonData, null, 2);
      toClipboard(text);
      btn.innerText = `✅ Copied ${jsonData.cards.length} cards to clipboard`;
      setTimeout(() => (btn.innerText = "📋 Copy Data"), 1500);
    });

    // Use insertBefore to put button as the first element
    container.insertBefore(btn, container.firstChild);
  }
};

main();
