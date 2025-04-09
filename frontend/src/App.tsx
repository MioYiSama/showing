import { lazy, LocationProvider, Route, Router } from "preact-iso";

export default () => (
  <LocationProvider>
    <Router>
      <Route path="/" component={lazy(() => import("./pages/Home"))} />
      <Route default component={lazy(() => import("./pages/NotFound"))} />
    </Router>
  </LocationProvider>
);
