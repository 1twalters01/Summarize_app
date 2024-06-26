import { lazy } from 'solid-js';
import { Route } from '@solidjs/router';

const LandingPage = lazy(() => import('./landing_page'));
const Pricing = lazy(() => import('./pricing'));

/** @returns {import("solid-js/types/jsx").JSX.Element} */
const LandingRoutes = () => {
  return (
    <>
      <Route path="/" component={LandingPage} />
      <Route path="/pricing" component={Pricing} />
    </>
  )
};

export default LandingRoutes;

