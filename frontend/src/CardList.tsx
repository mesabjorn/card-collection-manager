import { useEffect, useState } from "react";
import { getCards, type Card, updateCard } from "./services/cards.ts";

import "./App.css";

export function CardList({ seriesId }: { seriesId: number | null }) {
  
  const [initialCards, setInitialCards] = useState<Card[]>([]);
  const [visibleCards, setVisibleCards] = useState<Card[]>([]);
  const [search, setSearch] = useState("");

  const fetchCards = async (q?: string) => {
    const data = await getCards(q);
    const cards = data.map((a) => a[0]);
    setInitialCards(cards);
    setVisibleCards(cards);
  };

  useEffect(() => {
    fetchCards();
  }, []);

  useEffect(() => {
    handleSearch();
  }, [seriesId]);

  const handleSearch = async () => {
    // fetchCards(search);
    // if (search.length === 0) {
    //   setVisibleCards(initialCards);
    //   return;
    // }
    setVisibleCards(() => {
      return initialCards.filter((c) => {
        const matchesSearch = c.name
          .toLowerCase()
          .includes(search.toLowerCase());
        const matchesSeries = !seriesId || c.series_id==seriesId;
        return matchesSearch && matchesSeries;
      });
    });
  };

  const handleIncrement = async (card: Card) => {
    await updateCard(card.number, 1);
    fetchCards(search);
  };

  return (
    <div className="p-8">

      <div className="mb-4 flex gap-2">
        <input
          type="text"
          placeholder="Search by name"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="border rounded p-2 flex-1"
          onKeyUp={handleSearch}
        />
        <button
          onClick={handleSearch}
          className="bg-blue-500 text-white px-4 py-2 rounded"
        >
          Search
        </button>
      </div>

      <table className="table-auto w-full border-collapse border border-gray-300">
        <thead>
          <tr className="bg-gray-200">
            <th className="border p-2">Name</th>
            <th className="border p-2">Number</th>
            <th className="border p-2">In Collection</th>
            <th className="border p-2">Actions</th>
          </tr>
        </thead>
        <tbody>
          {visibleCards.map((card, i) => (
            <tr key={i} className="hover:bg-sky-700">
              <td className="border p-2">{card.name}</td>
              <td className="border p-2">{card.number}</td>
              <td className="border p-2">{card.in_collection}</td>
              <td className="border p-2 flex gap-2">
                <button
                  onClick={() => handleIncrement(card)}
                  className="bg-green-500 text-white px-2 rounded"
                >
                  +1
                </button>
                <button
                  // onClick={() => handleDelete(card)}
                  className="bg-red-500 text-white px-2 rounded"
                >
                  Delete
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
