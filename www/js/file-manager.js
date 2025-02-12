// File Manager Operations
const fileManager = {
  currentPath: "/",

  // Initialize file manager
  init() {
    this.bindEvents();
    this.loadFiles();
  },

  // Bind event listeners
  bindEvents() {
    // Operations dropdown
    const operationsMenu = document.getElementById("operations-menu");
    const operationsDropdown = document.getElementById("operations-dropdown");

    if (operationsMenu && operationsDropdown) {
      operationsMenu.addEventListener("click", (e) => {
        e.stopPropagation();
        const isVisible = operationsDropdown.style.display === "block";
        operationsDropdown.style.display = isVisible ? "none" : "block";
      });

      // Close dropdown when clicking outside
      document.addEventListener("click", (e) => {
        if (!operationsMenu.contains(e.target)) {
          operationsDropdown.style.display = "none";
        }
      });
    }

    // Upload file
    document.getElementById("uploadForm").addEventListener("submit", (e) => {
      e.preventDefault();
    });
    document
      .getElementById("uploadButton")
      .addEventListener("click", () => this.handleUpload());

    // Create directory
    document.getElementById("createDirForm").addEventListener("submit", (e) => {
      e.preventDefault();
    });
    document
      .getElementById("createDirButton")
      .addEventListener("click", () => this.handleCreateDirectory());

    // Navigation
    document.addEventListener("click", (e) => {
      if (e.target.closest("[data-path]")) {
        e.preventDefault();
        const path = e.target.closest("[data-path]").dataset.path;
        this.navigate(path);
      }
    });
  },

  // Load files for current directory
  async loadFiles() {
    try {
      const response = await fetch(
        `/api/files/${encodeURIComponent(this.currentPath)}`
      );
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const data = await response.json();
      if (!Array.isArray(data)) {
        throw new Error("Expected an array of files from the server");
      }
      this.renderFiles(data);
      this.updateBreadcrumb();
    } catch (error) {
      console.error("Error loading files:", error);
      this.showError("Failed to load files");
    }
  },

  // Handle file upload
  async handleUpload() {
    const fileInput = document.getElementById("fileInput");
    const file = fileInput.files[0];
    if (!file) {
      this.showError("Please select a file");
      return;
    }

    const formData = new FormData();
    formData.append("file", file);
    formData.append("path", this.currentPath);

    try {
      const response = await fetch("/api/files/upload", {
        method: "POST",
        body: formData,
      });

      const result = await response.json();
      if (!response.ok) throw new Error(result.error || "Upload failed");

      const modal = document.getElementById("uploadModal");
      const modalInstance = bootstrap.Modal.getInstance(modal);
      if (modalInstance) {
        modalInstance.hide();
        modal.addEventListener(
          "hidden.bs.modal",
          () => {
            fileInput.value = "";
          },
          { once: true }
        );
      }

      this.loadFiles();
      this.showSuccess("File uploaded successfully");
    } catch (error) {
      console.error("Error uploading file:", error);
      this.showError(error.message || "Failed to upload file");
    }
  },

  // Handle directory creation
  async handleCreateDirectory() {
    const dirNameInput = document.getElementById("dirName");
    const dirName = dirNameInput.value.trim();
    if (!dirName) {
      this.showError("Please enter a directory name");
      return;
    }

    try {
      const response = await fetch("/api/files/directory", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          path: this.currentPath,
          name: dirName,
        }),
      });

      const result = await response.json();
      if (!response.ok)
        throw new Error(result.error || "Failed to create directory");

      const modal = document.getElementById("createDirModal");
      const modalInstance = bootstrap.Modal.getInstance(modal);
      if (modalInstance) {
        modalInstance.hide();
        modal.addEventListener(
          "hidden.bs.modal",
          () => {
            dirNameInput.value = "";
          },
          { once: true }
        );
      }

      this.loadFiles();
      this.showSuccess("Directory created successfully");
    } catch (error) {
      console.error("Error creating directory:", error);
      this.showError(error.message || "Failed to create directory");
    }
  },

  // Handle file/directory deletion
  async handleDelete(path) {
    if (!confirm("Are you sure you want to delete this item?")) return;

    try {
      const response = await fetch(`/api/files/${encodeURIComponent(path)}`, {
        method: "DELETE",
      });

      if (!response.ok) throw new Error("Deletion failed");

      this.loadFiles();
      this.showSuccess("Item deleted successfully");
    } catch (error) {
      console.error("Error deleting item:", error);
      this.showError("Failed to delete item");
    }
  },

  // Navigate to directory
  navigate(path) {
    this.currentPath = path;
    this.loadFiles();
  },

  // Render file list
  renderFiles(files) {
    const tbody = document.getElementById("file-list");
    tbody.innerHTML = "";

    files.forEach((file) => {
      const tr = document.createElement("tr");
      tr.innerHTML = `
                <td class="px-6 py-4 whitespace-nowrap">
                    <div class="flex items-center">
                        <span class="text-sm font-medium text-gray-900">
                            ${
                              file.is_dir
                                ? `<a href="#" data-path="${file.path}" class="text-blue-600 hover:text-blue-800">${file.name}/</a>`
                                : file.name
                            }
                        </span>
                    </div>
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                    <span class="text-sm text-gray-500">${
                      file.is_dir ? "Directory" : "File"
                    }</span>
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                    <span class="text-sm text-gray-500">${this.formatSize(
                      file.size
                    )}</span>
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                    <span class="text-sm text-gray-500">${this.formatDate(
                      file.modified
                    )}</span>
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                    <button onclick="fileManager.handleDelete('${
                      file.path
                    }')" class="text-red-600 hover:text-red-900">Delete</button>
                </td>
            `;
      tbody.appendChild(tr);
    });
  },

  // Update breadcrumb navigation
  updateBreadcrumb() {
    const parts = this.currentPath.split("/").filter(Boolean);
    const ol = document.querySelector('nav[aria-label="Breadcrumb"] ol');

    // Keep only the root item
    while (ol.children.length > 1) {
      ol.removeChild(ol.lastChild);
    }

    let currentPath = "/";
    parts.forEach((part) => {
      currentPath += part + "/";
      const li = document.createElement("li");
      li.className = "inline-flex items-center";
      li.innerHTML = `
                <svg class="w-6 h-6 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
                </svg>
                <a href="#" data-path="${currentPath}" class="ml-1 text-sm font-medium text-blue-600 hover:text-blue-800">${part}</a>
            `;
      ol.appendChild(li);
    });
  },

  // Utility functions
  formatSize(bytes) {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  },

  formatDate(timestamp) {
    return new Date(parseInt(timestamp) * 1000).toLocaleString();
  },

  // Handle drag and drop file upload
  handleDrop(event) {
    event.preventDefault();
    const fileInput = document.getElementById("fileInput");
    const files = event.dataTransfer.files;
    if (files.length > 0) {
      fileInput.files = files;
    }
  },

  showSuccess(message) {
    // Implement toast or notification system
    alert(message);
  },

  showError(message) {
    // Implement toast or notification system
    alert(message);
  },
};

// Initialize file manager when DOM is loaded
document.addEventListener("DOMContentLoaded", () => fileManager.init());
