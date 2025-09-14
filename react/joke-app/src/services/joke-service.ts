import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react';
import type { Joke } from '../models/joke';

export const jokesApi = createApi({
  reducerPath: 'jokesApi',
  baseQuery: fetchBaseQuery({ baseUrl: 'https://api.chucknorris.io/jokes/' }),
  endpoints: (builder) => ({
    getRandomJoke: builder.query<Joke, void>({
      query: () => 'random',
    }),
  }),
});

export const { useGetRandomJokeQuery } = jokesApi;