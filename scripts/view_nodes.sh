#!/bin/bash
# vim: foldmethod=marker
#
# With a bunch of nodes running `docker compose up` run this script
# Then you can make nodes join the network and this script will autoreload the page
# Use ./scripts/run-demo.sh to automate network joining

mkdir -p scripts/web_view
cd scripts/web_view

# kill background jobs when script finishes
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

# index.html {{{
cat >index.html <<'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Overherd Network Monitor</title>
    <script type="text/javascript" src="https://unpkg.com/vis-network/standalone/umd/vis-network.min.js"></script>
    <style type="text/css">
        body, html { margin: 0; padding: 0; width: 100%; height: 100%; overflow: hidden; background-color: #1a1a1a; font-family: monospace; color: #eee; }
        #mynetwork { width: 100vw; height: 100vh; }

        .panel { background: rgba(0,0,0,0.85); padding: 15px; border-radius: 8px; border: 1px solid #444; position: absolute; z-index: 10; }
        #controls { top: 10px; left: 10px; display: flex; flex-direction: column; gap: 10px; }
        #leaderboard { top: 10px; right: 10px; width: 340px; max-height: 90vh; overflow-y: auto; }

        h3 { margin: 0 0 10px 0; color: #97c2fc; border-bottom: 1px solid #444; font-size: 14px; padding-bottom: 5px; }
        .stat-row { display: flex; justify-content: space-between; font-size: 11px; margin-bottom: 4px; border-bottom: 1px solid #222; padding-bottom: 2px; }
        .metric { color: #888; }

        button {
            background: #3a4a5e; color: white; border: 1px solid #555; padding: 8px 12px;
            border-radius: 4px; cursor: pointer; font-family: monospace;
        }
        button:hover { background: #4a5a6e; }
        button.active { background: #91735C; border-color: #97c2fc; }
    </style>
</head>
<body>
    <div id="controls" class="panel">
        <div style="font-weight:bold; color:#97c2fc">OVERHERD LIVE MAP</div>
        <button id="toggle-circle" onclick="toggleLayout()">MODE: ORGANIC</button>
        <div id="percentiles" style="font-size: 10px; color: #888;"></div>
    </div>

    <div id="leaderboard" class="panel">
        <h3>Node</h3>
        <div id="leaderboard-content">Scanning network...</div>
    </div>

    <div id="mynetwork"></div>

    <script type="text/javascript">
        let isCircleLayout = false;
        const nodes = new vis.DataSet([]);
        const edges = new vis.DataSet([]);

        const options = {
            nodes: { shape: 'dot', size: 15, font: { color: '#ffffff', size: 12, face: 'monospace' }, borderWidth: 2 },
            edges: {
                smooth: { enabled: true, type: 'continuous' },
                font: { size: 10, align: 'middle' }
            },
            physics: {
                enabled: true,
                solver: 'repulsion',
                repulsion: { nodeDistance: 150, centralGravity: 0.1 }
            }
        };

        const network = new vis.Network(document.getElementById('mynetwork'), { nodes, edges }, options);

        function toggleLayout() {
            isCircleLayout = !isCircleLayout;
            const btn = document.getElementById('toggle-circle');
            btn.innerText = `MODE: ${isCircleLayout ? 'CIRCLE' : 'ORGANIC'}`;
            btn.classList.toggle('active');
            network.setOptions({ physics: { enabled: !isCircleLayout } });
            updateGraph();
        }

        async function updateGraph() {
            try {
                const response = await fetch('graph.json?t=' + Date.now());
                const data = await response.json();

                // 1. Logic for Directions and Smart Arrows
                const connectionMap = new Map();
                data.edges.forEach(e => {
                    const id = [e.from, e.to].sort().join('-');
                    if (!connectionMap.has(id)) connectionMap.set(id, { pair: [e.from, e.to], directions: new Set() });
                    connectionMap.get(id).directions.add(`${e.from}->${e.to}`);
                });

                const currentEdgeIds = new Set();
                connectionMap.forEach((info, id) => {
                    currentEdgeIds.add(id);
                    const [nA, nB] = info.pair;
                    const aToB = info.directions.has(`${nA}->${nB}`);
                    const bToA = info.directions.has(`${nB}->${nA}`);
                    const isMutual = aToB && bToA;
                    const edgeUpdate = {
                        id: id,
                        from: nA,
                        to: nB,
                        arrows: {
                            to: { enabled: aToB, scaleFactor: 0.6 },
                            from: { enabled: bToA, scaleFactor: 0.6 }
                        },
                        color: {
                            // If it's a one-way connection, inherit color from the 'source' node
                            // If it's mutual, use a neutral fixed color
                            inherit: isMutual ? false : (aToB ? 'from' : 'to'),
                            color: isMutual ? '#4a5568' : undefined,
                            opacity: isMutual ? 0.8 : 0.4,
                            highlight: '#ffffff'
                        },
                        width: isMutual ? 2 : 1
                    };
                    if (edges.get(id)) edges.update(edgeUpdate); else edges.add(edgeUpdate);
                    });
                edges.getIds().forEach(id => { if (!currentEdgeIds.has(id)) edges.remove(id); });

                // 2. Node Stats & Muted Coloring
                const nodeStats = {};
                data.nodes.forEach(n => nodeStats[n.id] = { label: n.label, in: 0, out: 0, peers: new Set() });
                data.edges.forEach(e => {
                    if (nodeStats[e.from]) { nodeStats[e.from].out++; nodeStats[e.from].peers.add(e.to); }
                    if (nodeStats[e.to]) { nodeStats[e.to].in++; nodeStats[e.to].peers.add(e.from); }
                    });

                data.nodes.forEach((n, i) => {
                    const s = nodeStats[n.id];
                    const ratio = s.out / (s.in + s.out || 1);
                    let color = '#708090'; // Neutral
                    if (ratio > 0.6) color = '#5c7a91';      // Muted orange (Source)
                    else if (ratio < 0.4) color = '#8a9a5b'; // Muted Green (Sink)

                    const nodeUpdate = {
                        id: n.id, label: n.label,
                        color: { background: color, border: '#333' }
                    };

                    if (isCircleLayout) {
                        const angle = (i / data.nodes.length) * 2 * Math.PI;
                        const r = Math.max(250, data.nodes.length * 15);
                        nodeUpdate.x = r * Math.cos(angle);
                        nodeUpdate.y = r * Math.sin(angle);
                        nodeUpdate.fixed = true;
                    } else {
                        nodeUpdate.fixed = false;
                    }
                    nodes.update(nodeUpdate);
                });

                function getNodeColor(inCount, outCount) {
                    const total = inCount + outCount;
                    if (total === 0) return '#708090';
                    const ratio = outCount / total;
                    if (ratio > 0.6) return '#5c7a91'; // Blue
                    if (ratio < 0.4) return '#8a9a5b'; // Green
                    return '#4a5568'; // Balanced
                }

                const sortedNodes = Object.values(nodeStats).sort((a, b) => a.label.localeCompare(b.label));

                sortedNodes.forEach((s, i) => {
                    const color = getNodeColor(s.in, s.out);
                    const nodeUpdate = { id: s.id, label: s.label, color: { background: color, border: '#333' } };
                    if (isCircleLayout) {
                        const angle = (i / sortedNodes.length) * 2 * Math.PI;
                        const r = Math.max(250, sortedNodes.length * 15);
                        nodeUpdate.x = r * Math.cos(angle);
                        nodeUpdate.y = r * Math.sin(angle);
                        nodeUpdate.fixed = true;
                    } else {
                        nodeUpdate.fixed = false;
                    }
                    nodes.update(nodeUpdate);
                });

                const newNodeIds = data.nodes.map(n => n.id);
                nodes.getIds().forEach(id => { if (!newNodeIds.includes(id)) nodes.remove(id); });

                // 3. Leaderboard
                document.getElementById('leaderboard-content').innerHTML = sortedNodes.map(n => {
                    const bgColor = getNodeColor(n.in, n.out);
                    return `<div class="stat-row" style="background-color: ${bgColor}44; border-left: 4px solid ${bgColor}">
                        <span style="font-weight:bold">${n.label.split(' ')[0]}</span>
                        <span>
                            <span class="metric">I:</span><span class="val">${n.in}</span>
                            <span class="metric">O:</span><span class="val">${n.out}</span>
                        </span>
                    </div>`;
                }).join('');

            } catch (err) { console.error("Update loop error", err); }
        }

        setInterval(updateGraph, 1000);
        updateGraph();
    </script>
</body>
</html>
EOF
# }}}

python3 -m http.server 7777 2>/dev/null &
firefox --new-tab http://localhost:7777

containers=$(docker ps --filter "name=overherd-node-" --format "{{.ID}}")

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
        neighbors=$(cat "./out/${name}.dat")
        for neighbor_ip in $neighbors; do
            if [[ -n "$neighbor_ip" ]]; then
                edges_list+="$node_ip|$neighbor_ip "
            fi
        done
    done

    jq -n \
        --arg nodes_raw "$cached_nodes_list" \
        --arg edges_raw "$edges_list" \
        '{nodes: ($nodes_raw | split(";") | map(select(. != "") | split("|") | {id: .[0], label: .[1]})), edges: ($edges_raw | split(" ") | map(select(. != "") | split("|") | {from: .[0], to: .[1]}))}' > "graph.json"

    sleep 2
done
