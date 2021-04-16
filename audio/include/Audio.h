struct Audio {
    struct EventHandler {
        virtual void onSend(const std::string&, int) = 0;
        virtual int onReceive(const std::string&) = 0;
    };

    std::filesystem::path root;
    char* inputDeviceName;
    char* outputDeviceName;
    int voiceCount;

    void start(std::function<void(EventHandler*)>);
};
