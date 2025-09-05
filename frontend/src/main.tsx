import { StrictMode, useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import CardList from "./CardList.tsx";
import SeriesFilter from "./SeriesFilter.tsx";
import { getSeries, type Series } from "./services/cards.ts";

const App = () => {
  const [series, setSeries] = useState<Series[]>([]);
  const [selectedSeriesId, setSelectedSeriesId] = useState<number | null>(null);

  const handleChangeSelection = (id: number) => {
    const newSelection = id === selectedSeriesId ? null : id;
    setSelectedSeriesId(newSelection);
  };
  
  const fetchSeries = async () => {
      const data = await getSeries();
      console.log({series:data});
      setSeries(data);
  };

  useEffect(() => {
    fetchSeries();
  }, []);

  return (
    <>
      <div className="container mx-auto min-h-screen">
        <h1 className="text-2xl font-bold mb-4">Card Collection</h1>
        <div className="grid grid-cols-4">
          <SeriesFilter
            currentSelection={selectedSeriesId}
            series={series}
            onSelect={handleChangeSelection}
          />
          <CardList seriesId={selectedSeriesId} />
        </div>
      </div>
    </>
  );
};

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>
);
