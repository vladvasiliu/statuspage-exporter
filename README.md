[![License](https://img.shields.io/github/license/vladvasiliu/statuspage-exporter)](COPYING)
# statuspage-exporter

Exports service status from statuspage.io for Prometheus consumtion.


## Project status

The project is in its early stages of development, so breaking changes may happen at any time.


## !! WARNING !!

This program allows making requests to arbitrary URLs.
Make sur you understand the implications of deploying this in your infrastructure!

This exporter being similar to [blackbox_exporter](https://github.com/prometheus/blackbox_exporter), please read the
[exporters section](https://prometheus.io/docs/operating/security/#exporters) of the official docs.

If running on AWS, you may want to make sure you're using
[IMDSv2](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-instance-metadata-service.html) and limit the
role's permissions and network access to the absolute minimum.


## Running

Either build your own executable or using a Docker container:

```bash
docker run ghcr.io/vladvasiliu/statuspage-exporter:latest
```

It will listen on port 9925. It exposes two endpoints.


### `/probe` endpoint

This endpoint scrapes information from the statuspage.io domain you want to query. Please see the warning above.

It exports the overall status, the component status, and their respective latest update timestamp.

Example for Payline:

```
curl http://127.0.0.1:9925/probe?target=https://payline.statuspage.io/api/v2/summary.json

# HELP statuspage_component Per component status of this service, from the components element
# TYPE statuspage_component gauge
statuspage_component{component="3D Secure",status="degraded_performance"} 0
statuspage_component{component="3D Secure",status="major_outage"} 0
statuspage_component{component="3D Secure",status="operational"} 1
statuspage_component{component="3D Secure",status="partial_outage"} 0
statuspage_component{component="Back Office",status="degraded_performance"} 0
statuspage_component{component="Back Office",status="major_outage"} 0
statuspage_component{component="Back Office",status="operational"} 1
statuspage_component{component="Back Office",status="partial_outage"} 0
statuspage_component{component="File Transfer and Reporting",status="degraded_performance"} 0
statuspage_component{component="File Transfer and Reporting",status="major_outage"} 0
statuspage_component{component="File Transfer and Reporting",status="operational"} 1
statuspage_component{component="File Transfer and Reporting",status="partial_outage"} 0
statuspage_component{component="Fraud Detection Engine",status="degraded_performance"} 0
statuspage_component{component="Fraud Detection Engine",status="major_outage"} 0
statuspage_component{component="Fraud Detection Engine",status="operational"} 1
statuspage_component{component="Fraud Detection Engine",status="partial_outage"} 0
statuspage_component{component="Hosted Payment Pages",status="degraded_performance"} 0
statuspage_component{component="Hosted Payment Pages",status="major_outage"} 0
statuspage_component{component="Hosted Payment Pages",status="operational"} 1
statuspage_component{component="Hosted Payment Pages",status="partial_outage"} 0
statuspage_component{component="Payment Method",status="degraded_performance"} 0
statuspage_component{component="Payment Method",status="major_outage"} 0
statuspage_component{component="Payment Method",status="operational"} 1
statuspage_component{component="Payment Method",status="partial_outage"} 0
statuspage_component{component="Sandbox",status="degraded_performance"} 0
statuspage_component{component="Sandbox",status="major_outage"} 0
statuspage_component{component="Sandbox",status="operational"} 1
statuspage_component{component="Sandbox",status="partial_outage"} 0
statuspage_component{component="Web Service API",status="degraded_performance"} 0
statuspage_component{component="Web Service API",status="major_outage"} 0
statuspage_component{component="Web Service API",status="operational"} 1
statuspage_component{component="Web Service API",status="partial_outage"} 0
# HELP statuspage_component_timestamp Last update timestamp of the componet
# TYPE statuspage_component_timestamp gauge
statuspage_component_timestamp{component="3D Secure"} 1647378043
statuspage_component_timestamp{component="Back Office"} 1648063597
statuspage_component_timestamp{component="File Transfer and Reporting"} 1646802515
statuspage_component_timestamp{component="Fraud Detection Engine"} 1646802515
statuspage_component_timestamp{component="Hosted Payment Pages"} 1647378043
statuspage_component_timestamp{component="Payment Method"} 1647441520
statuspage_component_timestamp{component="Sandbox"} 1645629720
statuspage_component_timestamp{component="Web Service API"} 1647378043
# HELP statuspage_overall Overall status of this service, from the status element
# TYPE statuspage_overall gauge
statuspage_overall{indicator="critical"} 0
statuspage_overall{indicator="major"} 0
statuspage_overall{indicator="minor"} 0
statuspage_overall{indicator="none"} 1
# HELP statuspage_probe_success Whether all queries were successful
# TYPE statuspage_probe_success gauge
statuspage_probe_success 1
# HELP statuspage_timestamp Timestamp of last update of the status element
# TYPE statuspage_timestamp gauge
statuspage_timestamp 1648422629
```


### `/metrics` endpoint

This endpoint exports metrics about the application itself. Example:

```
# HELP process_cpu_seconds_total Total user and system CPU time spent in seconds.
# TYPE process_cpu_seconds_total counter
process_cpu_seconds_total 0.11
# HELP process_max_fds Maximum number of open file descriptors.
# TYPE process_max_fds gauge
process_max_fds 524288
# HELP process_open_fds Number of open file descriptors.
# TYPE process_open_fds gauge
process_open_fds 9
# HELP process_resident_memory_bytes Resident memory size in bytes.
# TYPE process_resident_memory_bytes gauge
process_resident_memory_bytes 13963264
# HELP process_start_time_seconds Start time of the process since unix epoch in seconds.
# TYPE process_start_time_seconds gauge
process_start_time_seconds 1649270761.41
# HELP process_threads Number of OS threads in the process.
# TYPE process_threads gauge
process_threads 1
# HELP process_virtual_memory_bytes Virtual memory size in bytes.
# TYPE process_virtual_memory_bytes gauge
process_virtual_memory_bytes 23048192
# HELP statuspage_info statuspage exporter version information
# TYPE statuspage_info gauge
statuspage_info{version="0.2.1"} 1
```


## Acknowledgements

This is inspired by [blackbox-exporter](https://github.com/prometheus/blackbox_exporter) for the
[multi-target exporter pattern](https://prometheus.io/docs/guides/multi-target-exporter/).


## License

This project is released under the terms of the GPL v3 License. Please refer to [`COPYING`](COPYING) for its terms.
