import type { Series } from "./services/cards";

type SeriesFilterProps = {
  series: Series[];
  currentSelection: number | null;
  onSelect: (s: number) => void;
};

const SeriesFilter = ({
  series,
  currentSelection,
  onSelect,
}: SeriesFilterProps) => {
  return (
    <>
      <ul className="space-y-2">
        <li className="text-xl font-bold mb-4">Series:</li>
        {series.map((s) => (
          <li
            key={s.id}
            onClick={() => onSelect(s.id)}
            className={`cursor-pointer rounded-2xl px-4 py-2 text-sm font-medium shadow-sm transition 
            ${
            s.id === currentSelection
            ? "bg-green-600 text-white shadow-md"
            : "bg-gray-100 text-gray-700 hover:bg-green-100 hover:text-green-700"
            }`}
            >
            {s.name}
          </li>
        ))}
      </ul>
    </>
  );
};

export default SeriesFilter;
