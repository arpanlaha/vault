import React, { ReactElement, useEffect } from "react";
import { Head } from "../components";
import { getCrate } from "../utils/api";

export default function Home(): ReactElement {
  useEffect(() => {
    const loadCrate = async (): Promise<void> => {
      const crate = await getCrate("serde");
      console.log(crate);
    };

    loadCrate();
  }, []);

  return (
    <>
      <Head />
      <div>Hello world!</div>
    </>
  );
}
