import { useEffect, useState, type JSX } from "react";
import {
  getCards,
  type Card,
  updateCard,
  type CardType,
  type Series,
} from "./services/cards.ts";
import { ChevronUp, ChevronDown, ShoppingCart, Copy } from "lucide-react";

import "./App.css";
import InputWithClearButton from "./InputWithClearButton.tsx";
import ToggleButtons from "./ToggleButtonsGroup.tsx";
import { toast } from "react-toastify";

function SelectAsCardMarketButton({
  data,
  series,
}: {
  series: Series | null;
  data: Card[] | null;
}) {
  // renders a copy to cardmarket button fixed to the bottom right
  // when a series is selected to create quick wants list of filtered cards

  const getFormattedData = (cards: Card[]) => {
    let result = "";
    for (const c of cards) {
      result += `${c.name} (${series?.name})\n`;
    }
    return result;
  };

  const copyToClipboard = async () => {
    if (navigator.clipboard && window.isSecureContext) {
      const promise = new Promise<JSX.Element>(async (resolve, reject) => {
        if (!data || data.length === 0) {
          reject("No data to copy.");
          return;
        }
        if (!series) {
          reject("No series selected.");
          return;
        }

        try {
          const formattedData = getFormattedData(data);
          await navigator.clipboard.writeText(formattedData);

          // Build JSX here and resolve it
          resolve(
            <div>
              <div>
                Copied {data.length} item{data.length !== 1 && "s"} to
                clipboard.
              </div>
              <a
                className="underline"
                href="https://www.cardmarket.com/en/YuGiOh/Wants"
                target="_blank"
                rel="noopener noreferrer"
              >
                Open CardMarket Wants
              </a>
            </div>
          );
        } catch (err) {
          reject("Failed to write to clipboard.");
        }
      });

      toast.promise(promise, {
        pending: "Formatting list...",
        success: {
          render({ data }) {
            return data; // <- this is your JSX
          },
        },
        error:
          "Error copying data: Make sure you selected one series and at least one card is visible",
      });
    } else {
      toast.error("Clipboard not supported in this browser/context.");
    }
  };

  if (!data || !series) return null;

  return (
    <div
      className="fixed right-0 bottom-0"
      title={`Copy current ${data.length} cards as a cardmarket wishlist format: {cardname} ({series name})`}
    >
      <button onClick={copyToClipboard}>
        <div className="flex gap-2 items-center">
          <Copy />
          CardMarket
        </div>
      </button>
    </div>
  );
}

export function CardList({ series }: { series: Series | null }) {
  const [initialCards, setInitialCards] = useState<Card[]>([]);
  const [visibleCards, setVisibleCards] = useState<Card[]>([]);
  const [search, setSearch] = useState("");
  const [sortConfig, setSortConfig] = useState<{
    key: keyof Card;
    direction: "asc" | "desc";
  } | null>({ key: "number", direction: "asc" });

  // Filter state: "all" | "collected" | "uncollected"
  const [collectionFilter, setCollectionFilter] = useState<
    "all" | "collected" | "uncollected"
  >("all");
  const [selectedRarities, setSelectedRarities] = useState<string[]>([]);

  const fetchCards = async (q?: string) => {
    const data = await getCards(q);
    setInitialCards(data);
    setVisibleCards(data);
  };

  useEffect(() => {
    fetchCards();
  }, []);

  useEffect(() => {
    setVisibleCards(
      initialCards.filter((c) => {
        const matchesSearch = c.name
          .toLowerCase()
          .includes(search.toLowerCase());
        const matchesSeries = !series || c.series.id === series.id;

        let matchesCollected = true;
        if (collectionFilter === "collected")
          matchesCollected = (c.in_collection ?? 0) > 0;
        else if (collectionFilter === "uncollected")
          matchesCollected = (c.in_collection ?? 0) === 0;

        const matchesRarity =
          selectedRarities.length === 0 ||
          selectedRarities.includes(c.rarity.name || "");

        return (
          matchesSearch && matchesSeries && matchesCollected && matchesRarity
        );
      })
    );
  }, [initialCards, search, series, collectionFilter, selectedRarities]);

  const handleIncrement = async (card: Card) => {
    //pass null to increment by one
    await updateCard(card.number, card.rarity, null);
    const newcards = initialCards.map((c) =>
      c.number === card.number && c.rarity.id === card.rarity.id
        ? { ...c, in_collection: (c.in_collection ?? 0) + 1 }
        : c
    );
    setInitialCards(newcards);
  };

  const handleDecrement = async (card: Card) => {
    await updateCard(card.number, card.rarity, -1);
    const newcards = initialCards.map((c) =>
      c.number === card.number && c.rarity.id === card.rarity.id
        ? { ...c, in_collection: (c.in_collection ?? 0) - 1 }
        : c
    );
    setInitialCards(newcards);
  };

  const sortBy = (key: keyof Card) => {
    let direction: "asc" | "desc" = "asc";

    if (
      sortConfig &&
      sortConfig.key === key &&
      sortConfig.direction === "asc"
    ) {
      direction = "desc";
    }

    const newCards = [...initialCards].sort((a, b) => {
      let aValue: string | number = "";
      let bValue: string | number = "";

      if (key === "rarity") {
        // special case: compare rarity.name
        aValue = a.rarity?.name ?? "";
        bValue = b.rarity?.name ?? "";
      } else {
        aValue = a[key] as string | number;
        bValue = b[key] as string | number;
      }

      let comparison = 0;
      if (typeof aValue === "string" && typeof bValue === "string") {
        comparison = aValue.localeCompare(bValue);
      } else if (typeof aValue === "number" && typeof bValue === "number") {
        comparison = aValue - bValue;
      }

      return direction === "asc" ? comparison : -comparison;
    });

    setInitialCards(newCards);
    setSortConfig({ key, direction });
  };

  const renderSortIcon = (key: keyof Card) => {
    if (!sortConfig || sortConfig.key !== key) return null;
    return sortConfig.direction === "asc" ? (
      <ChevronUp size={16} className="inline ml-1" />
    ) : (
      <ChevronDown size={16} className="inline ml-1" />
    );
  };

  const rarities = Array.from(
    new Set(
      initialCards
        .map((c) => c.rarity.name)
        .filter(Boolean)
        .sort()
    )
  ) as string[];

  const countCollected = () => {
    //count number of collected cards (ignoring copies);
    return visibleCards.reduce((acc, card) => {
      return acc + (card.in_collection && card.in_collection > 0 ? 1 : 0);
    }, 0);
  };

  const countNCards = () => {
    //count number of collected cards (including copies);
    return initialCards.reduce((acc, card) => {
      return acc + card.in_collection;
    }, 0);
  };

  const bgColorFromCardType = (cardtype: CardType): Record<string, String> => {
    let result = { color: "black", backgroundColor: "white" };

    switch (cardtype.sub) {
      case "Flip":
        result = { color: "black", backgroundColor: "#FF8B53" };
        break;

      case "Effect":
        result = { color: "black", backgroundColor: "#FF8B53" }; // Effect Monster (orange)
        break;
    }

    switch (cardtype.main) {
      case "Trap Card":
        result = { color: "black", backgroundColor: "#BC5A84" };
        break;

      case "Spell Card":
        result = { color: "black", backgroundColor: "#1d9e74" };
        break;

      case "Fusion Monster":
        // Special check: Effect Fusion Monster → orange

        result = { color: "black", backgroundColor: "#A086B7" };

        break;
      case "Ritual Monster":
        // Special check: Effect Fusion Monster → orange

        result = { color: "black", backgroundColor: "#9db5cc" };

        break;

      case "Monster":
        // Normal (non-effect) monsters → yellow
        if (cardtype.sub !== "Effect") {
          result = { color: "black", backgroundColor: "#FDE68A" };
        }
        if (cardtype.sub === "Flip" || cardtype.sub === "Toon") {
          result = { color: "black", backgroundColor: "#FF8B53" };
        }
        break;
    }

    return result;
  };

  const browseForCard = (card: Card) => {
    const propername = card.name.replaceAll(" ", "_");
    window.open(`https://yugioh.fandom.com/wiki/${propername}`);
  };

  function slugify(str: string) {
    return str
      .toLowerCase() // optional: normalize case
      .replace(/\s+/g, "-") // replace spaces with dashes
      .replace(/[^a-z0-9\-]/gi, "") // remove everything except letters, numbers, and dashes
      .replace(/-+/g, "-") // collapse multiple dashes
      .replace(/^-|-$/g, ""); // trim leading/trailing dashes
  }

  const getCardMarketRarityIdByRarityName = (name: string): number => {
    const mapping: Record<string, number> = {
      Common: 29,
      Rare: 28,
      "Super Rare": 3,
      "Ultra Rare": 4,
      "Secret Rare": 5,
      "Prismatic Secret Rare": 5,
      "Starlight Rare": 196,
      "Quarter Century Secret Rare": 292,
    };
    return mapping[name] ?? 0; // returns 0 if not found
  };

  const buyCard = (card: Card, seriesName: string): void => {
    const properSeriesName = slugify(seriesName);
    const cardMarketRarity = getCardMarketRarityIdByRarityName(
      card.rarity.name
    );

    const url = `https://www.cardmarket.com/en/YuGiOh/Products/Singles/${properSeriesName}?searchString=${card.name}&idRarity=${cardMarketRarity}&perSite=20`;
    console.log({ url });
    window.open(url);
  };

  return (
    <div className="p-8 col-span-3">
      {/* Filter bar */}
      <div className="mb-4 flex flex-col sm:flex-row gap-2 items-center">
        <InputWithClearButton
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          onClear={() => {
            setSearch("");
          }}
          placeholder="Find a card (by name)"
        />

        {/* Collection filter buttons */}
        <div className="flex gap-2">
          {(["all", "collected", "uncollected"] as const).map((filter) => (
            <button
              key={filter}
              onClick={() => setCollectionFilter(filter)}
              className={`px-3 py-1 rounded ${
                collectionFilter === filter
                  ? "bg-blue-500 text-white"
                  : "bg-gray-200 text-gray-500"
              }`}
            >
              {filter === "all"
                ? "Show All"
                : filter === "collected"
                ? "Show Collected"
                : "Show Uncollected"}
            </button>
          ))}
        </div>
      </div>
      <div className="flex justify-center w-full">
        <ToggleButtons
          options={rarities}
          selected={selectedRarities}
          onChange={setSelectedRarities}
        />
      </div>
      <div
        className="text-3xl font-bold"
        title={`Total cards in collection: ${countNCards()} (including duplicates)`}
      >{`${countCollected()}/${visibleCards.length} collected`}</div>
      <table className="table-auto w-full border-collapse border border-gray-300">
        <thead>
          <tr className="bg-green-500">
            <th
              className="border p-2 cursor-pointer"
              onClick={() => sortBy("name")}
            >
              Name {renderSortIcon("name")}
            </th>
            <th
              className="border p-2 cursor-pointer"
              onClick={() => sortBy("number")}
            >
              Number {renderSortIcon("number")}
            </th>
            <th
              className="border p-2 cursor-pointer"
              onClick={() => sortBy("in_collection")}
            >
              In Collection {renderSortIcon("in_collection")}
            </th>
            <th
              className="border p-2 cursor-pointer"
              onClick={() => sortBy("rarity")}
            >
              Rarity {renderSortIcon("rarity")}
            </th>
            <th
              className="border p-2 cursor-pointer"
              onClick={() => sortBy("cardtype_display")}
            >
              Card-Type {renderSortIcon("cardtype")}
            </th>
            <th className="border p-2">Actions</th>
          </tr>
        </thead>
        <tbody>
          {visibleCards.map((card, i) => (
            <tr
              key={i}
              className="hover:!bg-sky-400"
              style={bgColorFromCardType(card.cardtype)}
            >
              <td className="border p-2">
                <div
                  className="hover:underline cursor-pointer"
                  onClick={() => browseForCard(card)}
                >
                  {card.name}
                </div>
                {series && (
                  <ShoppingCart
                    size={16}
                    className="inline ml-1 hover:scale-125 cursor-pointer"
                    onClick={() => {
                      buyCard(card, series.name);
                    }}
                  ></ShoppingCart>
                )}
              </td>
              <td className="border p-2">{card.number}</td>
              <td className="border p-2">{card.in_collection}</td>
              <td className="border p-2">{card.rarity.name}</td>
              <td className="border p-2">{card.cardtype_display}</td>
              <td className="border p-2 flex gap-2">
                <button
                  onClick={() => handleDecrement(card)}
                  className="bg-green-500 text-white px-2 rounded"
                >
                  -1
                </button>
                <button
                  onClick={() => handleIncrement(card)}
                  className="bg-green-500 text-white px-2 rounded"
                >
                  +1
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      <SelectAsCardMarketButton data={visibleCards} series={series} />
    </div>
  );
}

export default CardList;
