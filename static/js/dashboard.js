(function () {
  const THEME_KEY = "inxi-dashboard-theme";
  const MODE_QUERY = "inxi-dashboard-mode";

  const modeSelect = document.getElementById("mode-select");
  const themeSelect = document.getElementById("theme-select");
  const refreshButton = document.getElementById("refresh-button");
  const statusText = document.getElementById("status-text");
  const componentCards = document.getElementById("component-cards");
  const downloadLink = document.getElementById("download-link");

  if (!modeSelect || !themeSelect || !refreshButton || !statusText || !componentCards) {
    return;
  }

  const state = {
    mode: modeSelect.value,
    theme: themeSelect.value,
  };

  const CARD_CONFIG = [
    {
      id: "system",
      label: "OS & Kernel",
      icon: "/static/icons/chip.png",
      sectionKeywords: ["system"],
      keywords: ["kernel", "desktop", "distro", "base", "arch"],
    },
    {
      id: "machine",
      label: "Machine",
      icon: "/static/icons/mainboard.png",
      sectionKeywords: ["machine", "mobo"],
      keywords: ["product", "vendor", "chassis", "serial", "uuid", "firmware", "bios"],
    },
    {
      id: "cpu",
      label: "CPU",
      icon: "/static/icons/chip.png",
      sectionKeywords: ["cpu"],
      keywords: ["cpu", "processor", "core", "thread", "cache", "clock", "ghz", "mhz"],
    },
    {
      id: "memory",
      label: "Memory",
      icon: "/static/icons/ssd.png",
      sectionKeywords: ["memory", "swap"],
      keywords: ["memory", "ram", "swap", "slot", "dimm", "channel", "ddr"],
    },
    {
      id: "storage",
      label: "SSD",
      icon: "/static/icons/ssd-drive.png",
      sectionKeywords: ["drives", "storage"],
      keywords: ["ssd", "nvme", "drive", "drives", "storage", "disk", "/dev/"],
    },
    {
      id: "gpu",
      label: "GPU",
      icon: "/static/icons/graphics-card.png",
      sectionKeywords: ["graphics", "display", "gpu", "video"],
      keywords: ["gpu", "graphics", "video", "vram", "nvidia", "radeon", "display", "vulkan"],
    },
    {
      id: "network",
      label: "Network",
      icon: "/static/icons/keyboard-and-mouse.png",
      sectionKeywords: ["network"],
      keywords: ["wlan", "ethernet", "wifi", "adapter", "driver", "if"],
    },
    {
      id: "battery",
      label: "Battery",
      icon: "/static/icons/mainboard.png",
      sectionKeywords: ["battery"],
      keywords: ["charge", "condition", "volts", "model", "li-poly", "charging"],
    },
    {
      id: "bluetooth",
      label: "Bluetooth",
      icon: "/static/icons/keyboard-and-mouse.png",
      sectionKeywords: ["bluetooth"],
      keywords: ["rfkill", "hci0", "driver", "btusb"],
    },
  ];

  function setTheme(theme) {
    document.documentElement.setAttribute("theme", theme);
    themeSelect.value = theme;
    state.theme = theme;
    localStorage.setItem(THEME_KEY, theme);
  }

  function loadTheme() {
    const stored = localStorage.getItem(THEME_KEY);
    if (stored) {
      setTheme(stored);
      return;
    }

    const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    setTheme(prefersDark ? "dark" : "default");
  }

  function setMode(mode) {
    state.mode = mode;
    modeSelect.value = mode;
    downloadLink.href = `/download?mode=${mode}`;
    localStorage.setItem(MODE_QUERY, mode);
  }

  function loadMode() {
    const stored = localStorage.getItem(MODE_QUERY);
    if (stored) {
      setMode(stored);
      return;
    }

    setMode(modeSelect.value);
  }

  function toggleLoading(isLoading) {
    refreshButton.disabled = isLoading;
    refreshButton.textContent = isLoading ? "Refreshing..." : "Refresh";
  }

  function entryLimitForMode(mode) {
    if (mode === "basic") return 5;
    if (mode === "full") return 7;
    if (mode === "verbose") return 9;
    return 11;
  }

  function copyToClipboard(text, contextNode) {
    if (navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(text).then(() => indicateCopy(contextNode));
      return;
    }

    const area = document.createElement("textarea");
    area.value = text;
    area.setAttribute("aria-hidden", "true");
    document.body.appendChild(area);
    area.select();
    document.execCommand("copy");
    document.body.removeChild(area);
    indicateCopy(contextNode);
  }

  function indicateCopy(target) {
    const hint = target.querySelector(".copy-hint");
    if (!hint) return;
    hint.classList.add("copy-confirmed");
    setTimeout(() => hint.classList.remove("copy-confirmed"), 1300);
  }

  function countKeywordMatches(text, keywords) {
    let score = 0;
    keywords.forEach((keyword) => {
      if (text.includes(keyword)) {
        score += 1;
      }
    });
    return score;
  }

  function classifyEntry(sectionTitle, entry) {
    const source = sectionTitle.toLowerCase();
    const key = entry.key.toLowerCase();
    const value = entry.value.toLowerCase();

    let bestCard = null;
    let bestScore = 0;

    CARD_CONFIG.forEach((card) => {
      let score = 0;

      if (card.sectionKeywords.some((keyword) => source.includes(keyword))) {
        score += 6;
      }

      score += countKeywordMatches(key, card.keywords) * 2;
      score += countKeywordMatches(value, card.keywords);

      if (score > bestScore) {
        bestScore = score;
        bestCard = card.id;
      }
    });

    if (bestScore < 2) {
      return null;
    }

    return bestCard;
  }

  function normalizeValue(value, key) {
    let cleaned = value.replace(/\s+/g, " ").trim();
    if (key) {
      const lowerKey = key.toLowerCase();
      if (cleaned.toLowerCase().startsWith(lowerKey + ":")) {
        cleaned = cleaned.substring(lowerKey.length + 1).trim();
      }
    }
    return cleaned;
  }

  function getCardPrimarySection(cardId) {
    if (cardId === "system") return "system";
    if (cardId === "machine") return "machine";
    if (cardId === "cpu") return "cpu";
    if (cardId === "memory") return "info";
    if (cardId === "storage") return "drives";
    if (cardId === "gpu") return "graphics";
    if (cardId === "network") return "network";
    if (cardId === "battery") return "battery";
    if (cardId === "bluetooth") return "bluetooth";
    return "";
  }

  function isLikelyGpuValue(value) {
    const normalized = value.toLowerCase();
    if (
      normalized.includes("camera") ||
      normalized.includes("uvcvideo") ||
      normalized.includes("webcam") ||
      normalized.includes("usb video")
    ) {
      return false;
    }

    return (
      normalized.includes("amd") ||
      normalized.includes("nvidia") ||
      normalized.includes("intel") ||
      normalized.includes("radeon") ||
      normalized.includes("geforce") ||
      normalized.includes("arc") ||
      normalized.includes("graphics") ||
      normalized.includes("display")
    );
  }

  function shouldKeepEntry(cardId, entry, mode) {
    const source = entry.source.toLowerCase();
    const key = entry.key.toLowerCase();
    const value = entry.value.toLowerCase();

    if (!entry.value || value.length < 2) return false;
    if (value.length > 200) return false;

    if (cardId === "system") {
      if (!source.includes("system")) return false;
      return true;
    }

    if (cardId === "machine") {
      if (!source.includes("machine") && !source.includes("mobo")) return false;
      return true;
    }

    if (cardId === "cpu") {
      if (!source.includes("cpu")) return false;
      if (key === "info" || key === "topology" || key.startsWith("speed")) return true;
      if (key.includes("cache")) return mode !== "basic";
      return false;
    }

    if (cardId === "memory") {
      const inMemorySection = source.includes("info") || source.includes("memory") || source.includes("swap");
      if (!inMemorySection) return false;
      if (value.includes("vram")) return false;
      if (source.includes("info")) return key === "memory";
      if (source.includes("swap")) return key.startsWith("id-");
      return true;
    }

    if (cardId === "storage") {
      if (!source.includes("drives")) return false;
      if (key === "local") return true;
      if (key.startsWith("id-")) return true;
      if (key === "temp" || key === "speed" || key.includes("smart")) return true;
      return false;
    }

    if (cardId === "gpu") {
      if (!source.includes("graphics")) return false;
      if (key === "info" || key === "x") return false;

      if (key.startsWith("device-")) {
        return isLikelyGpuValue(value);
      }

      if (key.startsWith("display") || key.startsWith("monitor-")) return true;

      if (key === "api") {
        if (mode !== "maximum") return false;
        if (value.includes("no vulkan data available")) return false;
        return value.includes("vulkan") || value.includes("opengl") || value.includes("egl");
      }

      return false;
    }

    if (cardId === "network") {
      if (!source.includes("network")) return false;
      return true;
    }

    if (cardId === "battery") {
      if (!source.includes("battery")) return false;
      return true;
    }

    if (cardId === "bluetooth") {
      if (!source.includes("bluetooth")) return false;
      return true;
    }

    return true;
  }

  function entryPriority(cardId, entry) {
    const key = entry.key.toLowerCase();
    const value = entry.value.toLowerCase();

    if (cardId === "cpu") {
      if (key === "info" && value.includes("model")) return 100;
      if (key === "topology") return 90;
      if (key.startsWith("speed")) return 80;
      if (key.includes("cache")) return 70;
      return 20;
    }

    if (cardId === "memory") {
      if (key === "memory") return 100;
      if (key.startsWith("id-")) return 80;
      return 20;
    }

    if (cardId === "storage") {
      if (key === "local") return 100;
      if (key.startsWith("id-")) return 80;
      return 20;
    }

    if (cardId === "gpu") {
      if (key.startsWith("device-")) return 100;
      if (key.startsWith("display")) return 90;
      if (key.startsWith("monitor-")) return 70;
      return 20;
    }

    return 0;
  }

  function metricLabel(cardId, entry) {
    const key = entry.key.toLowerCase();
    const source = entry.source.toLowerCase();

    if (cardId === "system") {
      if (key === "kernel") return "Kernel";
      if (key === "desktop") return "Desktop Environment";
      if (key === "distro") return "Distribution";
    }

    if (cardId === "machine") {
      if (key === "type") return "Form Factor";
      if (key === "system") return "Product Name";
      if (key === "mobo") return "Motherboard";
      if (key === "chassis") return "Chassis";
      if (key === "firmware") return "BIOS / UEFI";
    }

    if (cardId === "cpu") {
      if (key === "info") return "Model";
      if (key === "topology") return "Topology";
      if (key.startsWith("speed")) return "Speed";
    }

    if (cardId === "memory") {
      if (key === "memory") return "RAM";
      if (source.includes("swap") && key.startsWith("id-")) return "Swap";
    }

    if (cardId === "storage") {
      if (key === "local") return "Overall Usage";
      if (key.startsWith("id-")) return key.replace("id-", "Disk ");
      if (key === "temp") return "Temperature";
      if (key === "speed") return "Interface Speed";
      if (key.includes("smart")) return "Health Status";
    }

    if (cardId === "gpu") {
      if (key.startsWith("device-")) return key.replace("device-", "GPU ");
      if (key.startsWith("display")) return "Display";
      if (key.startsWith("monitor-")) return "Monitor";
      if (key === "api") return "API";
    }

    if (cardId === "network") {
      if (key.startsWith("device-")) return key.replace("device-", "Interface ");
      if (key === "if") return "Status";
    }

    if (cardId === "battery") {
      if (key.startsWith("id-")) return "Status";
      if (key === "charging") return "Charging Status";
    }

    return entry.key;
  }

  function curateEntries(cardId, entries, mode) {
    const seen = new Set();

    const curated = entries
      .map((entry) => ({
        source: entry.source,
        key: entry.key,
        value: normalizeValue(entry.value, entry.key),
      }))
      .filter((entry) => shouldKeepEntry(cardId, entry, mode))
      .filter((entry) => {
        const signature = `${entry.source.toLowerCase()}|${entry.key.toLowerCase()}|${entry.value.toLowerCase()}`;
        if (seen.has(signature)) return false;
        seen.add(signature);
        return true;
      })
      .sort((a, b) => entryPriority(cardId, b) - entryPriority(cardId, a));

    return curated;
  }

  function splitEntries(sections) {
    const groups = new Map();
    CARD_CONFIG.forEach((card) => groups.set(card.id, []));
    const seen = new Set();

    sections.forEach((section) => {
      section.entries.forEach((entry) => {
        const targetCard = classifyEntry(section.title, entry);
        if (!targetCard) return;

        const dedupeKey = `${targetCard}|${section.title}|${entry.key}|${entry.value}`;
        if (seen.has(dedupeKey)) return;
        seen.add(dedupeKey);

        groups.get(targetCard).push({
          source: section.title,
          key: entry.key,
          value: entry.value,
        });
      });
    });

    return groups;
  }

  function findEntry(entries, keywords) {
    return entries.find((entry) =>
      keywords.some((keyword) => {
        return (
          entry.key.toLowerCase().includes(keyword) ||
          entry.value.toLowerCase().includes(keyword)
        );
      })
    );
  }

  function buildCardSummary(cardId, entries) {
    if (!entries.length) {
      return "No matching data in current report.";
    }

    if (cardId === "memory") {
      const total = findEntry(entries, ["total", "memory", "ram"]);
      const system = findEntry(entries, ["used", "available", "active"]);
      const swap = findEntry(entries, ["swap", "id-"]);
      const parts = [];

      if (total) parts.push(`Total: ${total.value.split(" ")[0]}`);
      if (system) {
        const match = system.value.match(/used\s+([0-9.]+\s+\w+(?:\s+\([^)]+\))?)/i);
        if (match) parts.push(`Used: ${match[1]}`);
      }
      if (swap) {
        const match = swap.value.match(/used\s+([0-9.]+\s+\w+(?:\s+\([^)]+\))?)/i);
        if (match) parts.push(`Swap: ${match[1]}`);
      }
      if (parts.length) {
        return parts.join(" | ");
      }
    }

    if (cardId === "cpu") {
      const model = findEntry(entries, ["model", "processor", "name"]);
      const cores = findEntry(entries, ["core", "thread"]);
      const speed = findEntry(entries, ["speed", "mhz", "ghz"]);
      const parts = [];
      if (model) parts.push(model.value);
      if (cores) parts.push(cores.value);
      if (speed) parts.push(speed.value);
      if (parts.length) {
        return parts.join(" | ");
      }
    }

    if (cardId === "storage") {
      const usage = findEntry(entries, ["used", "total", "storage", "local"]);
      const disk = findEntry(entries, ["id-", "nvme", "ssd", "/dev/", "model"]);
      const health = findEntry(entries, ["smart", "temp", "health"]);
      const parts = [];

      if (disk) {
        const compact = disk.value.replace(/\s+/g, " ");
        const vendorMatch = compact.match(/\bvendor\s+(.+?)\s+\bmodel\b/i);
        const modelMatch = compact.match(/\bmodel\s+(.+?)\s+\bsize\b/i);
        const sizeMatch = compact.match(/\bsize\s+([0-9.]+\s+\w+)/i);
        
        if (vendorMatch && modelMatch) {
          parts.push(`${vendorMatch[1]} ${modelMatch[1]}`);
        } else if (modelMatch) {
          parts.push(modelMatch[1]);
        }
        
        if (sizeMatch) {
          parts.push(sizeMatch[1]);
        }
      }

      if (usage) {
        const compact = usage.value.replace(/\s+/g, " ");
        const percentMatch = compact.match(/\(([0-9.]+%)\)/);
        if (percentMatch) {
          parts.push(`Used: ${percentMatch[1]}`);
        }
      }

      if (health) {
        parts.push(health.value);
      }

      if (parts.length) {
        return parts.join(" | ");
      }
    }

    if (cardId === "gpu") {
      const model = findEntry(entries, ["model", "graphics", "card"]);
      const driver = findEntry(entries, ["driver"]);
      const vram = findEntry(entries, ["vram", "memory"]);
      const parts = [];
      if (model) parts.push(model.value);
      if (driver) parts.push(`Driver: ${driver.value}`);
      if (vram) parts.push(`VRAM: ${vram.value}`);
      if (parts.length) {
        return parts.join(" | ");
      }
    }

    return entries.slice(0, 2).map((entry) => entry.value).join(" | ");
  }

  function explodeValue(value) {
    const pairs = [];
    const regex = /(?:^|\s)([\w\(\)/-]+):/g;
    let match;
    let lastIndex = 0;
    let lastKey = null;

    const trimmedValue = value.trim();
    if (!trimmedValue.includes(":")) {
      return [{ key: "", value: trimmedValue }];
    }

    while ((match = regex.exec(value)) !== null) {
      if (lastKey !== null) {
        pairs.push({
          key: lastKey,
          value: value.substring(lastIndex, match.index).trim()
        });
      } else {
        const preText = value.substring(0, match.index).trim();
        if (preText) {
          pairs.push({ key: "", value: preText });
        }
      }
      lastKey = match[1];
      lastIndex = regex.lastIndex;
    }

    if (lastKey !== null) {
      pairs.push({
        key: lastKey,
        value: value.substring(lastIndex).trim()
      });
    }

    return pairs;
  }

  function buildTable(entries, cardId) {
    const table = document.createElement("table");
    table.className = "report-table";

    const thead = document.createElement("thead");
    thead.innerHTML = `
      <tr>
        <th>Technical Metric</th>
        <th>Value</th>
        <th class="text-right">Action</th>
      </tr>`;

    const tbody = document.createElement("tbody");

    entries.forEach((entry) => {
      const primarySection = getCardPrimarySection(cardId);
      const source = entry.source.toLowerCase();
      const mainLabel = metricLabel(cardId, entry);
      const displayLabel = (source === primarySection || source.includes(primarySection))
          ? mainLabel
          : `${entry.source} / ${mainLabel}`;

      const exploded = explodeValue(entry.value);

      if (exploded.length <= 1) {
        const row = document.createElement("tr");
        const keyCell = document.createElement("td");
        keyCell.className = "font-semibold text-muted";
        keyCell.textContent = displayLabel;

        const valueCell = document.createElement("td");
        valueCell.className = "entry-value";
        valueCell.textContent = exploded[0] ? exploded[0].value : entry.value;

        const actionCell = createActionCell(entry.value, row);
        
        row.appendChild(keyCell);
        row.appendChild(valueCell);
        row.appendChild(actionCell);
        tbody.appendChild(row);
      } else {
        const headRow = document.createElement("tr");
        headRow.className = "bg-surface";
        headRow.innerHTML = `<td colspan="3" class="font-bold text-xs uppercase tracking-wider py-2 px-4 border-b border-primary-light text-primary">${displayLabel}</td>`;
        tbody.appendChild(headRow);

        exploded.forEach((pair) => {
          const row = document.createElement("tr");
          const keyCell = document.createElement("td");
          keyCell.className = "font-semibold text-muted pl-4 text-xs italic";
          keyCell.textContent = pair.key ? pair.key : "—";

          const valueCell = document.createElement("td");
          valueCell.className = "entry-value";
          valueCell.textContent = pair.value;

          const actionCell = createActionCell(pair.value, row);
          
          row.appendChild(keyCell);
          row.appendChild(valueCell);
          row.appendChild(actionCell);
          tbody.appendChild(row);
        });
      }
    });

    table.appendChild(thead);
    table.appendChild(tbody);
    return table;
  }

  function createActionCell(text, row) {
    const actionCell = document.createElement("td");
    const copyButton = document.createElement("button");
    copyButton.className = "btn btn-outline btn-sm rounded-full";
    copyButton.style.padding = "2px 10px";
    copyButton.style.fontSize = "10px";
    copyButton.type = "button";
    copyButton.textContent = "Copy";
    copyButton.addEventListener("click", () => copyToClipboard(text, row));

    const hint = document.createElement("span");
    hint.className = "copy-hint text-success font-bold ml-2";
    hint.textContent = "✓";

    const copyWrap = document.createElement("div");
    copyWrap.className = "copy-cell";
    copyWrap.appendChild(copyButton);
    copyWrap.appendChild(hint);

    actionCell.appendChild(copyWrap);
    return actionCell;
  }

  function renderCard(card, allEntries, filteredCount) {
    const maxItems = entryLimitForMode(state.mode);
    const visibleEntries = allEntries.slice(0, maxItems);

    const article = document.createElement("article");
    article.className = "card component-card shadow-sm";

    const header = document.createElement("div");
    header.className = "component-card-header";

    const titleWrap = document.createElement("div");
    titleWrap.className = "component-title";

    const icon = document.createElement("img");
    icon.className = "insight-icon";
    icon.src = card.icon;
    icon.alt = `${card.label} icon`;

    const textWrap = document.createElement("div");
    textWrap.className = "insight-media";

    const title = document.createElement("h4");
    title.textContent = card.label;

    const meta = document.createElement("p");
    meta.className = "component-meta";
    if (filteredCount > 0) {
      meta.textContent = `${allEntries.length} items (${filteredCount} hidden)`;
    } else {
      meta.textContent = `${allEntries.length} items detected`;
    }

    textWrap.appendChild(title);
    textWrap.appendChild(meta);
    titleWrap.appendChild(icon);
    titleWrap.appendChild(textWrap);

    const copyAllButton = document.createElement("button");
    copyAllButton.className = "btn btn-secondary btn-sm rounded-full";
    copyAllButton.type = "button";
    copyAllButton.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-1"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg> Copy All`;
    copyAllButton.addEventListener("click", () => {
      const payload = allEntries
        .map((entry) => `${entry.source} / ${entry.key}: ${entry.value}`)
        .join("\n");
      copyToClipboard(payload, article);
    });

    header.appendChild(titleWrap);
    header.appendChild(copyAllButton);

    const body = document.createElement("div");
    body.className = "card-body p-0";

    if (!allEntries.length) {
      const empty = document.createElement("div");
      empty.className = "p-4 text-center text-muted italic";
      empty.textContent = "No data available in this mode.";
      body.appendChild(empty);
    } else {
      body.appendChild(buildTable(visibleEntries, card.id));
      if (visibleEntries.length < allEntries.length) {
        const note = document.createElement("div");
        note.className = "p-3 text-center text-xs text-muted border-t";
        note.textContent = `Showing ${visibleEntries.length} of ${allEntries.length} entries. Increase mode for more detail.`;
        body.appendChild(note);
      }
    }

    article.appendChild(header);
    article.appendChild(body);
    componentCards.appendChild(article);
  }

  function renderSections(report) {
    componentCards.innerHTML = "";
    const groups = splitEntries(report.sections);
    CARD_CONFIG.forEach((card) => {
      const rawEntries = groups.get(card.id) || [];
      const entries = curateEntries(card.id, rawEntries, state.mode);
      renderCard(card, entries, Math.max(rawEntries.length - entries.length, 0));
    });
  }

  function refreshReport() {
    toggleLoading(true);
    fetch(`/api/system?mode=${state.mode}`)
      .then((response) => {
        if (!response.ok) {
          return response.json().then((payload) => {
            throw new Error(payload.message || "Failed to fetch report");
          });
        }
        return response.json();
      })
      .then((payload) => {
        renderSections(payload);
        updateStatus(payload);
      })
      .catch((err) => {
        statusText.textContent = `Unable to refresh: ${err.message}`;
      })
      .finally(() => toggleLoading(false));
  }

  function updateStatus(report) {
    const millis = report.timestamp * 1000;
    const when = new Date(millis).toLocaleString();
    statusText.textContent = `Mode: ${report.mode} · Refreshed ${when}`;
  }

  themeSelect.addEventListener("change", (event) => {
    setTheme(event.target.value);
  });

  modeSelect.addEventListener("change", (event) => {
    setMode(event.target.value);
    refreshReport();
  });

  refreshButton.addEventListener("click", refreshReport);

  loadTheme();
  loadMode();
  refreshReport();
})();
