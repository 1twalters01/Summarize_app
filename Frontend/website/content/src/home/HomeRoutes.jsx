import { Route } from '@solidjs/router';

import Home from './pages/Home';


let HomeRoutes = () => {
  return (
    <>
      <Route path="home/" component={Home} />
    </>
  )
};

export default HomeRoutes;

