use anyhow::Context;
use crate::config::{FileRouteConfig, RouteConfig};
use crate::utils::get_current_folder;
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};
use tracing::instrument;

#[instrument(skip_all)]
pub fn register_routes(mut app: Router, routes: Vec<RouteConfig>) -> anyhow::Result<Router> {
    let base = get_current_folder().context("Unable to get app folder")?;

    for route in routes {
        match route {
            RouteConfig::File(FileRouteConfig {
                path,
                file,
                nest_routes,
            }) => {
                let file = base.join(file);
                app = app.route_service(&path, ServeFile::new(&file));
                tracing::debug!("RouteFile {} {} has been registered", path, file.display());
            }
            RouteConfig::Dir(url, dir) => {
                let path = base.join(&dir);
                app = app.route_service(&url, ServeDir::new(&path));
                tracing::debug!("RouteDir {} {} has been registered", url, path.display());
            }
            RouteConfig::FallbackFile(file) => {
                let path = base.join(file);
                app = app.fallback_service(ServeFile::new(&path));
                tracing::debug!("RouteFallbackFile {} has been registered", path.display());
            }
        }
    }

    Ok(app)
}
