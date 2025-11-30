# Nexus Industrial Graph Analytics

![Rust](https://img.shields.io/badge/backend-Rust-orange?style=flat-square&logo=rust)
![Neo4j](https://img.shields.io/badge/database-Neo4j-blue?style=flat-square&logo=neo4j)
![Architecture](https://img.shields.io/badge/architecture-Hexagonal-green?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-lightgrey?style=flat-square)

**[🇪🇸 Español](#-español) | [🇺🇸 English](#-english) | [🏴󠁥󠁳󠁣󠁴󠁿 Català](#-català)**

---

<a name="es"></a>
## 🇪🇸 Español

### Descripción General
**Nexus Industrial Graph Analytics** es una plataforma de alto rendimiento desarrollada en **Rust** diseñada para la gestión, visualización y análisis de activos industriales complejos. Utilizando **Neo4j** como motor de base de datos orientado a grafos, Nexus permite a ingenieros y gestores explorar relaciones entre equipos, materiales y ubicaciones técnicas en tiempo real.

El sistema implementa una **Arquitectura Hexagonal**, separando claramente la lógica de dominio (`nexus-core`), la infraestructura (`nexus-infra`) y la aplicación web (`nexus-app`), garantizando escalabilidad y mantenibilidad.

### 🚀 Funcionalidades Principales

#### 1. Visualización de Grafos Interactiva
*   **Motor Gráfico:** Renderizado dinámico de nodos y relaciones utilizando `Vis.js`.
*   **Exploración:** Permite expandir nodos, ver vecinos (padres/hijos) y detectar patrones visuales (bucles, islas, clusters).
*   **Jerarquías:** Visualización de árboles de montaje (BOM) multinivel y rutas críticas.

#### 2. Sistema de Herramientas Dinámico (`queries.json`)
El sistema es totalmente configurable mediante un archivo JSON que define las "Herramientas" disponibles sin recompilar el código:
*   **Consultas Cypher:** Ejecución segura de queries complejas a Neo4j (e.g., "Impacto de Obsolescencia", "Materiales Críticos").
*   **APIs Externas:** Integración con servicios HTTP para enriquecer datos (Clima, Precios de divisas, Datos químicos PubChem).

#### 3. Asistente IA con "Function Calling"
Integra un chat inteligente capaz de razonar sobre los datos industriales:
*   **Soporte Multi-Proveedor:** Compatible con OpenAI (GPT-4), Groq (Llama3) y Ollama (Local).
*   **Ejecución de Herramientas:** La IA puede invocar automáticamente las herramientas definidas (consultas a base de datos o APIs externas) para responder preguntas en lenguaje natural como *"¿Qué bombas tienen repuestos obsoletos?"*.

#### 4. Panel de Control (Dashboard)
*   **Búsqueda Predictiva:** Autocompletado rápido para encontrar activos por ID o nombre.
*   **Estadísticas:** Gráficos de barras automáticos generados con `Chart.js` basados en los datos recuperados.
*   **Exportación:** Descarga de resultados en CSV o capturas del grafo en PNG.

### 🛠️ Arquitectura Técnica
*   **Backend:** Rust (Actix-web, Tokio, Serde).
*   **Frontend:** HTML5, Bootstrap 5, Tera Templates (Server-Side Rendering).
*   **Base de Datos:** Neo4j (Driver `neo4rs`).
*   **Patrón:** Hexagonal (Ports & Adapters).

### ⚙️ Instalación y Uso

1.  **Prerrequisitos:**
    *   Rust (última versión estable).
    *   Una instancia de Neo4j (Local o AuraDB).

2.  **Configuración:**
    Crea un archivo `.env` en la raíz:
    ```env
    NEO4J_URI=bolt://localhost:7687
    NEO4J_USERNAME=neo4j
    NEO4J_PASSWORD=tu_password
    RUST_LOG=info
    ```

3.  **Ejecución:**
    ```bash
    cargo run -p nexus-app
    ```
    Accede a `http://localhost:8080`.

---

<a name="en"></a>
## 🇺🇸 English

### Overview
**Nexus Industrial Graph Analytics** is a high-performance platform built in **Rust**, designed for the management, visualization, and analysis of complex industrial assets. Leveraging **Neo4j** as a graph database engine, Nexus allows engineers and managers to explore relationships between equipment, materials, and technical locations in real-time.

The system implements a **Hexagonal Architecture**, clearly separating domain logic (`nexus-core`), infrastructure (`nexus-infra`), and the web application (`nexus-app`), ensuring scalability and maintainability.

### 🚀 Key Features

#### 1. Interactive Graph Visualization
*   **Graph Engine:** Dynamic rendering of nodes and relationships using `Vis.js`.
*   **Exploration:** Expand nodes, view neighbors (parents/children), and detect visual patterns (loops, islands, clusters).
*   **Hierarchies:** Visualization of multi-level Bill of Materials (BOM) and critical paths.

#### 2. Dynamic Tool System (`queries.json`)
The system is fully configurable via a JSON file that defines available "Tools" without recompiling the code:
*   **Cypher Queries:** Secure execution of complex queries to Neo4j (e.g., "Obsolescence Impact", "Critical Materials").
*   **External APIs:** Integration with HTTP services to enrich data (Weather, Currency Exchange, PubChem chemical data).

#### 3. AI Assistant with Function Calling
Integrates a smart chat interface capable of reasoning about industrial data:
*   **Multi-Provider Support:** Compatible with OpenAI (GPT-4), Groq (Llama3), and Ollama (Local).
*   **Tool Execution:** The AI can automatically invoke defined tools (database queries or external APIs) to answer natural language questions like *"Which pumps have obsolete spare parts?"*.

#### 4. Dashboard
*   **Predictive Search:** Fast autocomplete to find assets by ID or name.
*   **Statistics:** Automatic bar charts generated with `Chart.js` based on retrieved data.
*   **Export:** Download results as CSV or graph snapshots as PNG.

### 🛠️ Technical Architecture
*   **Backend:** Rust (Actix-web, Tokio, Serde).
*   **Frontend:** HTML5, Bootstrap 5, Tera Templates (Server-Side Rendering).
*   **Database:** Neo4j (`neo4rs` driver).
*   **Pattern:** Hexagonal (Ports & Adapters).

### ⚙️ Installation & Usage

1.  **Prerequisites:**
    *   Rust (latest stable version).
    *   A Neo4j instance (Local or AuraDB).

2.  **Configuration:**
    Create a `.env` file in the root directory:
    ```env
    NEO4J_URI=bolt://localhost:7687
    NEO4J_USERNAME=neo4j
    NEO4J_PASSWORD=your_password
    RUST_LOG=info
    ```

3.  **Run:**
    ```bash
    cargo run -p nexus-app
    ```
    Access at `http://localhost:8080`.

---

<a name="ca"></a>
## 🏴󠁥󠁳󠁣󠁴󠁿 Català

### Descripció General
**Nexus Industrial Graph Analytics** és una plataforma d'alt rendiment desenvolupada en **Rust**, dissenyada per a la gestió, visualització i anàlisi d'actius industrials complexos. Utilitzant **Neo4j** com a motor de base de dades orientat a grafs, Nexus permet a enginyers i gestors explorar relacions entre equips, materials i ubicacions tècniques en temps real.

El sistema implementa una **Arquitectura Hexagonal**, separant clarament la lògica de domini (`nexus-core`), la infraestructura (`nexus-infra`) i l'aplicació web (`nexus-app`), garantint escalabilitat i mantenibilitat.

### 🚀 Funcionalitats Principals

#### 1. Visualització de Grafs Interactiva
*   **Motor Gràfic:** Renderitzat dinàmic de nodes i relacions utilitzant `Vis.js`.
*   **Exploració:** Permet expandir nodes, veure veïns (pares/fills) i detectar patrons visuals (bucles, illes, clústers).
*   **Jerarquies:** Visualització d'arbres de muntatge (BOM) multinivell i rutes crítiques.

#### 2. Sistema d'Eines Dinàmic (`queries.json`)
El sistema és totalment configurable mitjançant un fitxer JSON que defineix les "Eines" disponibles sense recompilar el codi:
*   **Consultes Cypher:** Execució segura de queries complexes a Neo4j (p. ex., "Impacte d'Obsolescència", "Materials Crítics").
*   **APIs Externes:** Integració amb serveis HTTP per enriquir dades (Clima, Preus de divises, Dades químiques PubChem).

#### 3. Assistent IA amb "Function Calling"
Integra un xat intel·ligent capaç de raonar sobre les dades industrials:
*   **Suport Multi-Proveïdor:** Compatible amb OpenAI (GPT-4), Groq (Llama3) i Ollama (Local).
*   **Execució d'Eines:** La IA pot invocar automàticament les eines definides (consultes a base de dades o APIs externes) per respondre preguntes en llenguatge natural com *"Quines bombes tenen recanvis obsolets?"*.

#### 4. Tauler de Control (Dashboard)
*   **Cerca Predictiva:** Emplenament automàtic ràpid per trobar actius per ID o nom.
*   **Estadístiques:** Gràfics de barres automàtics generats amb `Chart.js` basats en les dades recuperades.
*   **Exportació:** Descàrrega de resultats en CSV o captures del graf en PNG.

### 🛠️ Arquitectura Tècnica
*   **Backend:** Rust (Actix-web, Tokio, Serde).
*   **Frontend:** HTML5, Bootstrap 5, Tera Templates (Server-Side Rendering).
*   **Base de Dades:** Neo4j (Driver `neo4rs`).
*   **Patró:** Hexagonal (Ports & Adapters).

### ⚙️ Instal·lació i Ús

1.  **Requisits previs:**
    *   Rust (última versió estable).
    *   Una instància de Neo4j (Local o AuraDB).

2.  **Configuració:**
    Crea un fitxer `.env` a l'arrel:
    ```env
    NEO4J_URI=bolt://localhost:7687
    NEO4J_USERNAME=neo4j
    NEO4J_PASSWORD=el_teu_password
    RUST_LOG=info
    ```

3.  **Execució:**
    ```bash
    cargo run -p nexus-app
    ```
    Accedeix a `http://localhost:8080`.