import { useEffect, useState } from "react";
import { getCards, type Card, updateCard } from "./services/cards.ts";
import { ChevronUp, ChevronDown } from "lucide-react";

import "./App.css";
import InputWithResetButton from "./InputWithClearButton.tsx";

export function CardList({ seriesId }: { seriesId: number | null }) {
  const [initialCards, setInitialCards] = useState<Card[]>([]);
  const [visibleCards, setVisibleCards] = useState<Card[]>([]);
  const [search, setSearch] = useState("");
  const [sortConfig, setSortConfig] = useState<{ key: keyof Card; direction: "asc" | "desc" } | null>(null);
  
  // Filter state: "all" | "collected" | "uncollected"
  const [collectionFilter, setCollectionFilter] = useState<"all" | "collected" | "uncollected">("all");
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
        const matchesSearch = c.name.toLowerCase().includes(search.toLowerCase());
        const matchesSeries = !seriesId || c.series_id === seriesId;

        let matchesCollected = true;
        if (collectionFilter === "collected") matchesCollected = (c.in_collection ?? 0) > 0;
        else if (collectionFilter === "uncollected") matchesCollected = (c.in_collection ?? 0) === 0;

        const matchesRarity =
          selectedRarities.length === 0 || selectedRarities.includes(c.rarity || "");

        return matchesSearch && matchesSeries && matchesCollected && matchesRarity;
      })
    );
  }, [initialCards, search, seriesId, collectionFilter, selectedRarities]);

  const handleIncrement = async (card: Card) => {
    await updateCard(card.number, 1);
    const newcards = initialCards.map((c) =>
      c.number === card.number ? { ...c, in_collection: (c.in_collection ?? 0) + 1 } : c
    );
    setInitialCards(newcards);
  };

  const sortBy = (key: keyof Card) => {
    let direction: "asc" | "desc" = "asc";
    if (sortConfig && sortConfig.key === key && sortConfig.direction === "asc") direction = "desc";

    const newcards = [...initialCards].sort((a, b) => {
      const aValue = a[key] ?? "";
      const bValue = b[key] ?? "";

      let comparison = 0;
      if (typeof aValue === "string" && typeof bValue === "string") comparison = aValue.localeCompare(bValue);
      else if (typeof aValue === "number" && typeof bValue === "number") comparison = aValue - bValue;

      return direction === "asc" ? comparison : -comparison;
    });

    setInitialCards(newcards);
    setSortConfig({ key, direction });
  };

  const renderSortIcon = (key: keyof Card) => {
    if (!sortConfig || sortConfig.key !== key) return null;
    return sortConfig.direction === "asc" ? <ChevronUp size={16} className="inline ml-1" /> : <ChevronDown size={16} className="inline ml-1" />;
  };

  const rarities = Array.from(new Set(initialCards.map((c) => c.rarity).filter(Boolean))) as string[];

  const countCollected = ()=>{
    //count number of collected cards (ignoring copies);
    return visibleCards.reduce((acc,card)=>{
      return acc + (card.in_collection && card.in_collection>0?1:0);
    },0)
  }

  return (
    <div className="p-8 col-span-3">
      {/* Filter bar */}
      <div className="mb-4 flex flex-col sm:flex-row gap-2 items-center">
        <InputWithResetButton
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          onClear={()=>{setSearch('')}}
        />
        
        

        {/* Collection filter buttons */}
        <div className="flex gap-2">
          {(["all", "collected", "uncollected"] as const).map((filter) => (
            <button
              key={filter}
              onClick={() => setCollectionFilter(filter)}
              className={`px-3 py-1 rounded ${
                collectionFilter === filter ? "bg-blue-500 text-white" : "bg-gray-200 text-gray-700"
              }`}
            >
              {filter === "all" ? "Show All" : filter === "collected" ? "Show Collected" : "Show Uncollected"}
            </button>
          ))}
        </div>

        {/* Rarity filter dropdown */}
        <select
          multiple
          value={selectedRarities}
          onChange={(e) =>
            setSelectedRarities(Array.from(e.target.selectedOptions, (o) => o.value))
          }
          className="border rounded p-2"
        >
          {rarities.map((r) => (
            <option key={r} value={r}>
              {r}
            </option>
          ))}
        </select>
      </div>
      <div className="text-3xl font-bold">{`${countCollected()}/${visibleCards.length} collected`}</div>
      <table className="table-auto w-full border-collapse border border-gray-300">
        <thead>
          <tr className="bg-green-500">
            <th className="border p-2 cursor-pointer" onClick={() => sortBy("name")}>
              Name {renderSortIcon("name")}
            </th>
            <th className="border p-2 cursor-pointer" onClick={() => sortBy("number")}>
              Number {renderSortIcon("number")}
            </th>
            <th className="border p-2 cursor-pointer" onClick={() => sortBy("in_collection")}>
              In Collection {renderSortIcon("in_collection")}
            </th>
            <th className="border p-2 cursor-pointer" onClick={() => sortBy("rarity")}>
              Rarity {renderSortIcon("rarity")}
            </th>
            <th className="border p-2">Actions</th>
          </tr>
        </thead>
        <tbody>
          {visibleCards.map((card, i) => (
            <tr key={i} className="hover:bg-sky-700">
              <td className="border p-2">{card.name}</td>
              <td className="border p-2">{card.number}</td>
              <td className="border p-2">{card.in_collection}</td>
              <td className="border p-2">{card.rarity}</td>
              <td className="border p-2 flex gap-2">
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
    </div>
  );
}

export default CardList;
