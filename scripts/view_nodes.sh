#!/bin/bash
# vim: foldmethod=marker
#
# With a bunch of nodes running `docker compose up` run this script
# Then you can make nodes join the network and this script will autoreload the page
# Use ./scripts/run-demo.sh to automate network joining

OUTPUT_FILE="graph.json"

mkdir -p scripts/web_view
cd scripts/web_view

# kill background jobs when script finishes
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

# index.html {{{
cat >index.html <<EOF
<!DOCTYPE html>
<html lang="en">
<head>
		<meta charset="UTF-8">
		<title>Overherd Network Monitor</title>
		<script type="text/javascript" src="https://unpkg.com/vis-network/standalone/umd/vis-network.min.js"></script>
		<style type="text/css">
				body, html { margin: 0; padding: 0; width: 100%; height: 100%; overflow: hidden; background-color: #1a1a1a; }
				#mynetwork { width: 100vw; height: 100vh; }
				#status-overlay {
						position: absolute; top: 10px; left: 10px; color: #ffffff;
						font-family: monospace; z-index: 10; pointer-events: none;
						background: rgba(0,0,0,0.5); padding: 10px; border-radius: 5px;
					}
		</style>
</head>
<body>
		<div id="status-overlay">LIVE NETWORK MAP | Updating every 1s</div>
		<div id="mynetwork"></div>

		<script type="text/javascript">
				const container = document.getElementById('mynetwork');
				const nodes = new vis.DataSet([]);
				const edges = new vis.DataSet([]);
				const data = { nodes, edges };
				const options = {
						nodes: {
								shape: 'dot', size: 20,
								font: { color: '#ffffff', size: 14, face: 'monospace' },
								borderWidth: 2,
								color: { background: '#97c2fc', border: '#2B7CE9' }
							},
						edges: {
								arrows: { to: { enabled: true, scaleFactor: 1 } },
								color: { color: '#666666', highlight: '#ffffff' },
								width: 1
							},
						physics: {
								enabled: true,
								solver: 'repulsion',
								repulsion: { nodeDistance: 150, centralGravity: 0.2 },
								stabilization: { iterations: 100, updateInterval: 10 }
							}
						};
				const network = new vis.Network(container, data, options);
				async function updateGraph() {
						try {
								const response = await fetch('${OUTPUT_FILE}?t=' + Date.now());
								const newData = await response.json();
								const newNodeIds = newData.nodes.map(n => n.id);
								newData.nodes.forEach(newNode => {
										const existingNode = nodes.get(newNode.id);
										if (!existingNode || existingNode.label !== newNode.label || existingNode.color !== newNode.color) {
												nodes.update(newNode);
											}
										});
								nodes.getIds().forEach(id => {
										if (!newNodeIds.includes(id)) nodes.remove(id);
										});
								const currentEdgeIds = new Set(newData.edges.map(e => \`\${e.from}-\${e.to}\`));
								const existingEdgeIds = new Set(edges.getIds());
								newData.edges.forEach(edge => {
										const edgeId = \`\${edge.from}-\${edge.to}\`;
										if (!existingEdgeIds.has(edgeId)) {
												edges.add({ id: edgeId, from: edge.from, to: edge.to });
											}
										});
								existingEdgeIds.forEach(id => {
										if (!currentEdgeIds.has(id)) {
												edges.remove(id);
											}
										});
									} catch (err) {
								console.error("Fetch error:", err);
							}
						}
				updateGraph();
				setInterval(updateGraph, 1000);
		</script>
</body>
</html>
EOF
# }}}

python3 -m http.server 7777 2>/dev/null &
firefox --new-tab http://localhost:7777

containers=$(docker ps --filter "name=overherd-node-" --format "{{.Names}}")

declare -A container_ips
cached_nodes_list=""

for name in $containers; do
	node_ip=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "$name")
	if [[ -n "$node_ip" ]]; then
		container_ips["$name"]="$node_ip"
		cached_nodes_list+="$node_ip|$name ($node_ip);"
	fi
done

while true; do
	edges_list=""
	for name in "${!container_ips[@]}"; do
		node_ip=${container_ips[$name]}
		neighbors=$(docker exec "$name" cat list.txt 2>/dev/null || echo "")
		for neighbor_ip in $neighbors; do
			if [[ -n "$neighbor_ip" ]]; then
				edges_list+="$node_ip|$neighbor_ip "
			fi
		done
	done

	jq -n \
		--arg nodes_raw "$cached_nodes_list" \
		--arg edges_raw "$edges_list" \
		'{nodes: ($nodes_raw | split(";") | map(select(. != "") | split("|") | {id: .[0], label: .[1]})), edges: ($edges_raw | split(" ") | map(select(. != "") | split("|") | {from: .[0], to: .[1]}))}' > "$OUTPUT_FILE"

	sleep 2
done
