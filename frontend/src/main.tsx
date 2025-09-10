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
    
    setSeries(data);
  };

  useEffect(() => {
    fetchSeries();
  }, []);

  return (
    <>
      <div className="container mx-auto min-h-screen">
        <h1 className="text-2xl font-bold mb-4">Card Collection</h1>

        {/* Parent grid with 2 columns */}
        <div className="grid grid-cols-12 gap-4">
          {/* Left column */}
          <div className="col-span-3">
            <SeriesFilter
              currentSelection={selectedSeriesId}
              series={series}
              onSelect={handleChangeSelection}
            />
          </div>

          {/* Right column */}
          <div className="col-span-9">
            <CardList seriesId={selectedSeriesId} />
          </div>
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
