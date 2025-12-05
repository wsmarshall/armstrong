A back-end oriented, cloud native application containerized in Docker and intended to be continuously deployed. Approached
as a production focused, API development practice process.

Uses trunk-based development along with CI/CD practices, and (from C. Davis via L. Palmieri) expects to achieve availability in fault-prone environments,
continual release of new versions (no downtime), and able to handle dynamic loads.

Relies on actix-web for the web framework, tokio for the asynchronous runtime, and sqlx for PostgreSQL database interfacing/management.

All the thanks and gratitude to Luca Palmieri for guidance along the way.
