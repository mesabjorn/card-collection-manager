import axios from "axios";

export const api = axios.create({
  baseURL: "http://localhost:3000/api", // your Axum backend
});

export interface Card {
  name: string;
  series_id: number;
  number: string;
  collection_number: number;
  in_collection: number;
  rarity_id: number;
  card_type_id: number;
}

export async function getCards(query?: string) {
  if (query) {
    const res = await api.post<Card[]>("/cards", { name: query });
    return res.data;
  }
  const res = await api.get<Card[]>("/cards");
  return res.data;
}

export async function updateCard(id: String, number: number) {
  const res = await api.put<number>("/cards", {id});
  return res.data;
}


