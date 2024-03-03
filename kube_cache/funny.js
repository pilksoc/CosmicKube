const kubes = "https://hack.djpiper28.co.uk/cache/kubes";

fetch(kubes)
  .then((res) => res.json())
  .then(async (data) => {
    for (let i = 0; i < data.length; i++) {
      for (let j = 0; j < i; j++) {
        const url =
          "https://hack.djpiper28.co.uk/cache/kubeRecipeByIds/" +
          data[i].id +
          "/" +
          data[j].id;
        console.log(url);
        await fetch(url, {
          timeout: 1000,
        })
          .then((res) => console.log(res.json))
          .catch(async (err) => {
            console.error(err);
            await new Promise((resolve) => setTimeout(resolve, 10000));
          });
      }
    }
  });
