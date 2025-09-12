import axios from "axios";

export const api = axios.create({
  baseURL: "http://localhost:3000/api", // your Axum backend
});

export interface Card {
  name: string;
  number: string;
  collection_number: number;
  in_collection: number;
  series: Series;
  rarity: Rarity;
  cardtype: CardType;
  cardtype_display: String;
}

export interface Rarity {
  id: string;
  name: string;
}

export interface CardType {
  main:String;
  sub:String;  
}

export interface Series {
  name: string;
  id:number;
  n_cards:number;
  prefix:string;
  release_date:string;
}

export async function getCards(query?: string) {
  if (query) {
    const res = await api.post<Card[]>("/cards", { name: query });
    return res.data;
  }
  const res = await api.get<Card[]>("/cards");
  return res.data;
}

export async function updateCard(id: String, number: number|null) {
  const res = await api.put<number>("/cards", {id,number});
  return res.data;
}

export async function getSeries() {
  const res = await api.get<Series[]>("/series");
  return res.data;
}

