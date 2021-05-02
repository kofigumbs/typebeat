struct Audio {
    struct EventHandler {
        virtual void onSend(const std::string&, int) = 0;
        virtual bool onReceive(const std::string&, int&) = 0;
        virtual void drop(int, const void*) = 0;
    };

    std::filesystem::path root;
    char* inputDeviceName;
    char* outputDeviceName;
    int voiceCount;
    dsp* insert;

    void start(std::function<void(EventHandler*)>);
};
