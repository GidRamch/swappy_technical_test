import { useGetRandomJokeQuery } from './services/joke-service';


function App() {

  const { data, error, isLoading } = useGetRandomJokeQuery();

  return (
    <>
      {
        error
          ? <div>There was an error loading the joke. Please try again.</div>
          : isLoading
            ? <div>Loading joke...</div>
            : <div>{data?.value ?? 'Joke value is empty'}</div>
      }
    </>
  )
}

export default App
