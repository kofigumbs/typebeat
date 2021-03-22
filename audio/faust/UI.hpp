struct UI {
    Destinations* destinations;
    int voice;

    void openTabBox(const char* label) {
        voice = std::stoi(label);
    }

    void addNumEntry(const char* label, FAUSTFLOAT* zone, FAUSTFLOAT init, FAUSTFLOAT min, FAUSTFLOAT max, FAUSTFLOAT step) {
        destinations->add(voice, std::string(label), zone, min, max);
    }

    void closeBox() {}
    void openVerticalBox(const char* label) {}
    void declare(FAUSTFLOAT*, const char*, const char*) {}
};
