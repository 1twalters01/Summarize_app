import { lazy } from 'solid-js';
import { Route } from '@solidjs/router';

const Landing = lazy(() => import('./landing'));
const Pricing = lazy(() => import('./pricing'));

/** @returns {import("solid-js/types/jsx").JSX.Element} */
const LandingRoutes = () => {
  return (
    <>
      <Route path="/" component={Landing} />
      <Route path="/pricing" component={Pricing} />
    </>
  )
};

export default LandingRoutes;

